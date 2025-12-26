use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DiagCtxtInner {
    fn new(emitter: Box<DynEmitter>) -> Self {
        Self {
            flags: DiagCtxtFlags {
                can_emit_warnings: true,
                ..Default::default()
            },
            registry: Registry::new(&[]),
            err_guars: Vec::new(),
            lint_err_guars: Vec::new(),
            delayed_bugs: Vec::new(),
            deduplicated_err_count: 0,
            deduplicated_warn_count: 0,
            emitter,
            must_produce_diag: None,
            has_printed: false,
            suppressed_expected_diag: false,
            taught_diagnostics: Default::default(),
            emitted_diagnostic_codes: Default::default(),
            emitted_diagnostics: Default::default(),
            stashed_diagnostics: Default::default(),
            future_breakage_diagnostics: Vec::new(),
            fulfilled_expectations: Default::default(),
            ice_file: None,
        }
    }
    /// Emit all stashed diagnostics.
    fn emit_stashed_diagnostics(&mut self) -> Option<ErrorGuaranteed> {
        let mut guar = None;
        let has_errors = !self.err_guars.is_empty();
        for (_, stashed_diagnostics) in std::mem::take(&mut self.stashed_diagnostics).into_iter() {
            for (_, (diag, _guar)) in stashed_diagnostics {
                if !diag.is_error() {
                    if !diag.is_force_warn() && has_errors {
                        continue;
                    }
                }
                guar = guar.or(self.emit_diagnostic(diag, None));
            }
        }
        guar
    }
    fn emit_diagnostic(
        &mut self,
        mut diagnostic: DiagInner,
        taint: Option<&Cell<Option<ErrorGuaranteed>>>,
    ) -> Option<ErrorGuaranteed> {
        if diagnostic.has_future_breakage() {
            assert_matches!(
                diagnostic.level,
                Error | ForceWarning | Warning | Allow | Expect
            );
            self.future_breakage_diagnostics.push(diagnostic.clone());
        }
        match diagnostic.level {
            Bug => {}
            Fatal | Error => {
                if self.treat_next_err_as_bug() {
                    diagnostic.level = Bug;
                }
            }
            DelayedBug => {
                if self.flags.eagerly_emit_delayed_bugs {
                    if self.treat_next_err_as_bug() {
                        diagnostic.level = Bug;
                    } else {
                        diagnostic.level = Error;
                    }
                } else {
                    return if let Some(guar) = self.has_errors() {
                        Some(guar)
                    } else {
                        let backtrace = std::backtrace::Backtrace::capture();
                        #[allow(deprecated)]
                        let guar = ErrorGuaranteed::unchecked_error_guaranteed();
                        self.delayed_bugs.push((
                            DelayedDiagInner::with_backtrace(diagnostic, backtrace),
                            guar,
                        ));
                        Some(guar)
                    };
                }
            }
            ForceWarning if diagnostic.lint_id.is_none() => {}
            Warning => {
                if !self.flags.can_emit_warnings {
                    if diagnostic.has_future_breakage() {
                        TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
                    }
                    return None;
                }
            }
            Note | Help | FailureNote => {}
            OnceNote | OnceHelp => panic!("bad level: {:?}", diagnostic.level),
            Allow => {
                if diagnostic.has_future_breakage() {
                    TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
                    self.suppressed_expected_diag = true;
                }
                return None;
            }
            Expect | ForceWarning => {
                self.fulfilled_expectations
                    .insert(diagnostic.lint_id.unwrap());
                if let Expect = diagnostic.level {
                    TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
                    self.suppressed_expected_diag = true;
                    return None;
                }
            }
        }
        TRACK_DIAGNOSTIC(diagnostic, &mut |mut diagnostic| {
            if let Some(code) = diagnostic.code {
                self.emitted_diagnostic_codes.insert(code);
            }
            let already_emitted = {
                let mut hasher = StableHasher::new();
                diagnostic.hash(&mut hasher);
                let diagnostic_hash = hasher.finish();
                !self.emitted_diagnostics.insert(diagnostic_hash)
            };
            let is_error = diagnostic.is_error();
            let is_lint = diagnostic.is_lint.is_some();
            if !(self.flags.deduplicate_diagnostics && already_emitted) {
                debug!(?diagnostic);
                debug!(? self.emitted_diagnostics);
                let not_yet_emitted = |sub: &mut Subdiag| {
                    debug!(?sub);
                    if sub.level != OnceNote && sub.level != OnceHelp {
                        return true;
                    }
                    let mut hasher = StableHasher::new();
                    sub.hash(&mut hasher);
                    let diagnostic_hash = hasher.finish();
                    debug!(?diagnostic_hash);
                    self.emitted_diagnostics.insert(diagnostic_hash)
                };
                diagnostic.children.retain_mut(not_yet_emitted);
                if already_emitted {
                    let msg = "duplicate diagnostic emitted due to `-Z deduplicate-diagnostics=no`";
                    diagnostic.sub(Note, msg, MultiSpan::new());
                }
                if is_error {
                    self.deduplicated_err_count += 1;
                } else if matches!(diagnostic.level, ForceWarning | Warning) {
                    self.deduplicated_warn_count += 1;
                }
                self.has_printed = true;
                self.emitter.emit_diagnostic(diagnostic, &self.registry);
            }
            if is_error {
                if !self.delayed_bugs.is_empty() {
                    assert_eq!(self.lint_err_guars.len() + self.err_guars.len(), 0);
                    self.delayed_bugs.clear();
                    self.delayed_bugs.shrink_to_fit();
                }
                #[allow(deprecated)]
                let guar = ErrorGuaranteed::unchecked_error_guaranteed();
                if is_lint {
                    self.lint_err_guars.push(guar);
                } else {
                    if let Some(taint) = taint {
                        taint.set(Some(guar));
                    }
                    self.err_guars.push(guar);
                }
                self.panic_if_treat_err_as_bug();
                Some(guar)
            } else {
                None
            }
        })
    }
    fn treat_err_as_bug(&self) -> bool {
        self.flags
            .treat_err_as_bug
            .is_some_and(|c| self.err_guars.len() + self.lint_err_guars.len() >= c.get())
    }
    fn treat_next_err_as_bug(&self) -> bool {
        self.flags
            .treat_err_as_bug
            .is_some_and(|c| self.err_guars.len() + self.lint_err_guars.len() + 1 >= c.get())
    }
    fn has_errors_excluding_lint_errors(&self) -> Option<ErrorGuaranteed> {
        self.err_guars.get(0).copied().or_else(|| {
            if let Some((_diag, guar)) = self
                .stashed_diagnostics
                .values()
                .flat_map(|stashed_diagnostics| stashed_diagnostics.values())
                .find(|(diag, guar)| guar.is_some() && diag.is_lint.is_none())
            {
                *guar
            } else {
                None
            }
        })
    }
    fn has_errors(&self) -> Option<ErrorGuaranteed> {
        self.err_guars
            .get(0)
            .copied()
            .or_else(|| self.lint_err_guars.get(0).copied())
            .or_else(|| {
                self.stashed_diagnostics
                    .values()
                    .find_map(|stashed_diagnostics| {
                        stashed_diagnostics.values().find_map(|(_, guar)| *guar)
                    })
            })
    }
    fn has_errors_or_delayed_bugs(&self) -> Option<ErrorGuaranteed> {
        self.has_errors()
            .or_else(|| self.delayed_bugs.get(0).map(|(_, guar)| guar).copied())
    }
    /// Translate `message` eagerly with `args` to `SubdiagMessage::Eager`.
    fn eagerly_translate<'a>(
        &self,
        message: DiagMessage,
        args: impl Iterator<Item = DiagArg<'a>>,
    ) -> SubdiagMessage {
        SubdiagMessage::Translated(Cow::from(self.eagerly_translate_to_string(message, args)))
    }
    /// Translate `message` eagerly with `args` to `String`.
    fn eagerly_translate_to_string<'a>(
        &self,
        message: DiagMessage,
        args: impl Iterator<Item = DiagArg<'a>>,
    ) -> String {
        let args = crate::translation::to_fluent_args(args);
        self.emitter
            .translator()
            .translate_message(&message, &args)
            .map_err(Report::new)
            .unwrap()
            .to_string()
    }
    fn eagerly_translate_for_subdiag(
        &self,
        diag: &DiagInner,
        msg: impl Into<SubdiagMessage>,
    ) -> SubdiagMessage {
        let msg = diag.subdiagnostic_message_to_diagnostic_message(msg);
        self.eagerly_translate(msg, diag.args.iter())
    }
    fn flush_delayed(&mut self) {
        assert!(self.stashed_diagnostics.is_empty());
        if !self.err_guars.is_empty() {
            return;
        }
        if self.delayed_bugs.is_empty() {
            return;
        }
        let bugs: Vec<_> = std::mem::take(&mut self.delayed_bugs)
            .into_iter()
            .map(|(b, _)| b)
            .collect();
        let backtrace = std::env::var_os("RUST_BACKTRACE").as_deref() != Some(OsStr::new("0"));
        let decorate = backtrace || self.ice_file.is_none();
        let mut out = self.ice_file.as_ref().and_then(|file| {
            std::fs::File::options()
                .create(true)
                .append(true)
                .open(file)
                .ok()
        });
        let note1 = "no errors encountered even though delayed bugs were created";
        let note2 = "those delayed bugs will now be shown as internal compiler errors";
        self.emit_diagnostic(DiagInner::new(Note, note1), None);
        self.emit_diagnostic(DiagInner::new(Note, note2), None);
        for bug in bugs {
            if let Some(out) = &mut out {
                _ = write!(
                    out,
                    "delayed bug: {}\n{}\n",
                    bug.inner
                        .messages
                        .iter()
                        .filter_map(|(msg, _)| msg.as_str())
                        .collect::<String>(),
                    &bug.note
                );
            }
            let mut bug = if decorate {
                bug.decorate(self)
            } else {
                bug.inner
            };
            if bug.level != DelayedBug {
                bug.arg("level", bug.level);
                let msg = crate::fluent_generated::errors_invalid_flushed_delayed_diagnostic_level;
                let msg = self.eagerly_translate_for_subdiag(&bug, msg);
                bug.sub(Note, msg, bug.span.primary_span().unwrap().into());
            }
            bug.level = Bug;
            self.emit_diagnostic(bug, None);
        }
        panic::panic_any(DelayedBugPanic);
    }
    fn panic_if_treat_err_as_bug(&self) {
        if self.treat_err_as_bug() {
            let n = self.flags.treat_err_as_bug.map(|c| c.get()).unwrap();
            assert_eq!(n, self.err_guars.len() + self.lint_err_guars.len());
            if n == 1 {
                panic!("aborting due to `-Z treat-err-as-bug=1`");
            } else {
                panic!("aborting after {n} errors due to `-Z treat-err-as-bug={n}`");
            }
        }
    }
}
