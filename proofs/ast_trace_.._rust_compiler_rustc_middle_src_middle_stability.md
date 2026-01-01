# AST Trace: ../rust/compiler/rustc_middle/src/middle/stability.rs

Generated 24 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
//! A pass that annotates every item and method with its stability level,
//! propagating default levels lexically from parent to children ast nodes.

use std::num::NonZero;

use rustc_ast::NodeId;
use rustc_errors::{Applicability, Diag, EmissionGuarantee, LintBuffer};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_feature::GateIssue;
use rustc_hir::attrs::{DeprecatedSince, Deprecation};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_hir::def_id::{DefId, LocalDefId};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_hir::{self as hir, ConstStability, DefaultBodyStability, HirId, Stability};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_macros::{Decodable, Encodable, HashStable, Subdiagnostic};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_session::Session;
use rustc_session::lint::builtin::{DEPRECATED, DEPRECATED_IN_FUTURE, SOFT_UNSTABLE};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_session::lint::{BuiltinLintDiag, DeprecatedSinceKind, Level, Lint};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_session::parse::feature_err_issue;
use rustc_span::{Span, Symbol, sym};
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
use tracing::debug;

pub use self::StabilityLevel::*;
use crate::ty::TyCtxt;
use crate::ty::print::with_no_trimmed_paths;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum StabilityLevel {
    Unstable,
    Stable,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Copy, Clone)]
pub enum UnstableKind {
    /// Enforcing regular stability of an item
    Regular,
    /// Enforcing const stability of an item
    Const(Span),
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=DeprecationEntry | COMPLEXITY=4 | LINES=10

```rust
/// An entry in the `depr_map`.
#[derive(Copy, Clone, HashStable, Debug, Encodable, Decodable)]
pub struct DeprecationEntry {
    /// The metadata of the attribute associated with this entry.
    pub attr: Deprecation,
    /// The `DefId` where the attr was originally attached. `None` for non-local
    /// `DefId`'s.
    origin: Option<LocalDefId>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=local | COMPLEXITY=11 | LINES=17

```rust
impl DeprecationEntry {
    pub fn local(attr: Deprecation, def_id: LocalDefId) -> DeprecationEntry {
        DeprecationEntry { attr, origin: Some(def_id) }
    }

    pub fn external(attr: Deprecation) -> DeprecationEntry {
        DeprecationEntry { attr, origin: None }
    }

    pub fn same_origin(&self, other: &DeprecationEntry) -> bool {
        match (self.origin, other.origin) {
            (Some(o1), Some(o2)) => o1 == o2,
            _ => false,
        }
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=report_unstable | COMPLEXITY=27 | LINES=35

```rust
pub fn report_unstable(
    sess: &Session,
    feature: Symbol,
    reason: Option<Symbol>,
    issue: Option<NonZero<u32>>,
    suggestion: Option<(Span, String, String, Applicability)>,
    is_soft: bool,
    span: Span,
    soft_handler: impl FnOnce(&'static Lint, Span, String),
    kind: UnstableKind,
) {
    let qual = match kind {
        UnstableKind::Regular => "",
        UnstableKind::Const(_) => " const",
    };

    let msg = match reason {
        Some(r) => format!("use of unstable{qual} library feature `{feature}`: {r}"),
        None => format!("use of unstable{qual} library feature `{feature}`"),
    };

    if is_soft {
        soft_handler(SOFT_UNSTABLE, span, msg)
    } else {
        let mut err = feature_err_issue(sess, feature, span, GateIssue::Library(issue), msg);
        if let Some((inner_types, msg, sugg, applicability)) = suggestion {
            err.span_suggestion(inner_types, msg, sugg, applicability);
        }
        if let UnstableKind::Const(kw) = kind {
            err.span_label(kw, "trait is not stable as const yet");
        }
        err.emit();
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=deprecation_lint | COMPLEXITY=6 | LINES=4

```rust
fn deprecation_lint(is_in_effect: bool) -> &'static Lint {
    if is_in_effect { DEPRECATED } else { DEPRECATED_IN_FUTURE }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=DeprecationSuggestion | COMPLEXITY=3 | LINES=15

```rust
#[derive(Subdiagnostic)]
#[suggestion(
    middle_deprecated_suggestion,
    code = "{suggestion}",
    style = "verbose",
    applicability = "machine-applicable"
)]
pub struct DeprecationSuggestion {
    #[primary_span]
    pub span: Span,

    pub kind: String,
    pub suggestion: Symbol,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=Deprecated | COMPLEXITY=2 | LINES=10

```rust
pub struct Deprecated {
    pub sub: Option<DeprecationSuggestion>,

    // FIXME: make this translatable
    pub kind: String,
    pub path: String,
    pub note: Option<Symbol>,
    pub since_kind: DeprecatedSinceKind,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=decorate_lint | COMPLEXITY=22 | LINES=26

```rust
impl<'a, G: EmissionGuarantee> rustc_errors::LintDiagnostic<'a, G> for Deprecated {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, G>) {
        diag.primary_message(match &self.since_kind {
            DeprecatedSinceKind::InEffect => crate::fluent_generated::middle_deprecated,
            DeprecatedSinceKind::InFuture => crate::fluent_generated::middle_deprecated_in_future,
            DeprecatedSinceKind::InVersion(_) => {
                crate::fluent_generated::middle_deprecated_in_version
            }
        });
        diag.arg("kind", self.kind);
        diag.arg("path", self.path);
        if let DeprecatedSinceKind::InVersion(version) = self.since_kind {
            diag.arg("version", version);
        }
        if let Some(note) = self.note {
            diag.arg("has_note", true);
            diag.arg("note", note);
        } else {
            diag.arg("has_note", false);
        }
        if let Some(sub) = self.sub {
            diag.subdiagnostic(sub);
        }
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=deprecated_since_kind | COMPLEXITY=14 | LINES=18

```rust
fn deprecated_since_kind(is_in_effect: bool, since: DeprecatedSince) -> DeprecatedSinceKind {
    if is_in_effect {
        DeprecatedSinceKind::InEffect
    } else {
        match since {
            DeprecatedSince::RustcVersion(version) => {
                DeprecatedSinceKind::InVersion(version.to_string())
            }
            DeprecatedSince::Future => DeprecatedSinceKind::InFuture,
            DeprecatedSince::NonStandard(_)
            | DeprecatedSince::Unspecified
            | DeprecatedSince::Err => {
                unreachable!("this deprecation is always in effect; {since:?}")
            }
        }
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=early_report_macro_deprecation | COMPLEXITY=7 | LINES=22

```rust
pub fn early_report_macro_deprecation(
    lint_buffer: &mut LintBuffer,
    depr: &Deprecation,
    span: Span,
    node_id: NodeId,
    path: String,
) {
    if span.in_derive_expansion() {
        return;
    }

    let is_in_effect = depr.is_in_effect();
    let diag = BuiltinLintDiag::DeprecatedMacro {
        suggestion: depr.suggestion,
        suggestion_span: span,
        note: depr.note,
        path,
        since_kind: deprecated_since_kind(is_in_effect, depr.since),
    };
    lint_buffer.buffer_lint(deprecation_lint(is_in_effect), node_id, span, diag);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=late_report_deprecation | COMPLEXITY=20 | LINES=42

```rust
fn late_report_deprecation(
    tcx: TyCtxt<'_>,
    depr: &Deprecation,
    span: Span,
    method_span: Option<Span>,
    hir_id: HirId,
    def_id: DefId,
) {
    if span.in_derive_expansion() {
        return;
    }

    let is_in_effect = depr.is_in_effect();
    let lint = deprecation_lint(is_in_effect);

    // Calculating message for lint involves calling `self.def_path_str`,
    // which will by default invoke the expensive `visible_parent_map` query.
    // Skip all that work if the lint is allowed anyway.
    if tcx.lint_level_at_node(lint, hir_id).level == Level::Allow {
        return;
    }

    let def_path = with_no_trimmed_paths!(tcx.def_path_str(def_id));
    let def_kind = tcx.def_descr(def_id);

    let method_span = method_span.unwrap_or(span);
    let suggestion =
        if let hir::Node::Expr(_) = tcx.hir_node(hir_id) { depr.suggestion } else { None };
    let diag = Deprecated {
        sub: suggestion.map(|suggestion| DeprecationSuggestion {
            span: method_span,
            kind: def_kind.to_owned(),
            suggestion,
        }),
        kind: def_kind.to_owned(),
        path: def_path,
        note: depr.note,
        since_kind: deprecated_since_kind(is_in_effect, depr.since),
    };
    tcx.emit_node_span_lint(lint, hir_id, method_span, diag);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=18

```rust
/// Result of `TyCtxt::eval_stability`.
pub enum EvalResult {
    /// We can use the item because it is stable or we provided the
    /// corresponding feature gate.
    Allow,
    /// We cannot use the item because it is unstable and we did not provide the
    /// corresponding feature gate.
    Deny {
        feature: Symbol,
        reason: Option<Symbol>,
        issue: Option<NonZero<u32>>,
        suggestion: Option<(Span, String, String, Applicability)>,
        is_soft: bool,
    },
    /// The item does not have the `#[stable]` or `#[unstable]` marker assigned.
    Unmarked,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=suggestion_for_allocator_api | COMPLEXITY=16 | LINES=26

```rust
// See issue #83250.
fn suggestion_for_allocator_api(
    tcx: TyCtxt<'_>,
    def_id: DefId,
    span: Span,
    feature: Symbol,
) -> Option<(Span, String, String, Applicability)> {
    if feature == sym::allocator_api {
        if let Some(trait_) = tcx.opt_parent(def_id) {
            if tcx.is_diagnostic_item(sym::Vec, trait_) {
                let sm = tcx.sess.psess.source_map();
                let inner_types = sm.span_extend_to_prev_char(span, '<', true);
                if let Ok(snippet) = sm.span_to_snippet(inner_types) {
                    return Some((
                        inner_types,
                        "consider wrapping the inner types in tuple".to_string(),
                        format!("({snippet})"),
                        Applicability::MaybeIncorrect,
                    ));
                }
            }
        }
    }
    None
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=6 | LINES=8

```rust
/// An override option for eval_stability.
pub enum AllowUnstable {
    /// Don't emit an unstable error for the item
    Yes,
    /// Handle the item normally
    No,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=eval_stability | COMPLEXITY=183 | LINES=356

```rust
impl<'tcx> TyCtxt<'tcx> {
    /// Evaluates the stability of an item.
    ///
    /// Returns `EvalResult::Allow` if the item is stable, or unstable but the corresponding
    /// `#![feature]` has been provided. Returns `EvalResult::Deny` which describes the offending
    /// unstable feature otherwise.
    ///
    /// If `id` is `Some(_)`, this function will also check if the item at `def_id` has been
    /// deprecated. If the item is indeed deprecated, we will emit a deprecation lint attached to
    /// `id`.
    pub fn eval_stability(
        self,
        def_id: DefId,
        id: Option<HirId>,
        span: Span,
        method_span: Option<Span>,
    ) -> EvalResult {
        self.eval_stability_allow_unstable(def_id, id, span, method_span, AllowUnstable::No)
    }

    /// Evaluates the stability of an item.
    ///
    /// Returns `EvalResult::Allow` if the item is stable, or unstable but the corresponding
    /// `#![feature]` has been provided. Returns `EvalResult::Deny` which describes the offending
    /// unstable feature otherwise.
    ///
    /// If `id` is `Some(_)`, this function will also check if the item at `def_id` has been
    /// deprecated. If the item is indeed deprecated, we will emit a deprecation lint attached to
    /// `id`.
    ///
    /// Pass `AllowUnstable::Yes` to `allow_unstable` to force an unstable item to be allowed. Deprecation warnings will be emitted normally.
    pub fn eval_stability_allow_unstable(
        self,
        def_id: DefId,
        id: Option<HirId>,
        span: Span,
        method_span: Option<Span>,
        allow_unstable: AllowUnstable,
    ) -> EvalResult {
        // Deprecated attributes apply in-crate and cross-crate.
        if let Some(id) = id {
            if let Some(depr_entry) = self.lookup_deprecation_entry(def_id) {
                let parent_def_id = self.hir_get_parent_item(id);
                let skip = self
                    .lookup_deprecation_entry(parent_def_id.to_def_id())
                    .is_some_and(|parent_depr| parent_depr.same_origin(&depr_entry));

                // #[deprecated] doesn't emit a notice if we're not on the
                // topmost deprecation. For example, if a struct is deprecated,
                // the use of a field won't be linted.
                //
                // With #![staged_api], we want to emit down the whole
                // hierarchy.
                let depr_attr = &depr_entry.attr;
                if !skip || depr_attr.is_since_rustc_version() {
                    late_report_deprecation(self, depr_attr, span, method_span, id, def_id);
                }
            };
        }

        let is_staged_api = self.lookup_stability(def_id.krate.as_def_id()).is_some();
        if !is_staged_api {
            return EvalResult::Allow;
        }

        // Only the cross-crate scenario matters when checking unstable APIs
        let cross_crate = !def_id.is_local();
        if !cross_crate {
            return EvalResult::Allow;
        }

        let stability = self.lookup_stability(def_id);
        debug!(
            "stability: \
                inspecting def_id={:?} span={:?} of stability={:?}",
            def_id, span, stability
        );

        match stability {
            Some(Stability {
                level: hir::StabilityLevel::Unstable { reason, issue, is_soft, implied_by, .. },
                feature,
                ..
            }) => {
                if span.allows_unstable(feature) {
                    debug!("stability: skipping span={:?} since it is internal", span);
                    return EvalResult::Allow;
                }
                if self.features().enabled(feature) {
                    return EvalResult::Allow;
                }

                // If this item was previously part of a now-stabilized feature which is still
                // enabled (i.e. the user hasn't removed the attribute for the stabilized feature
                // yet) then allow use of this item.
                if let Some(implied_by) = implied_by
                    && self.features().enabled(implied_by)
                {
                    return EvalResult::Allow;
                }

                // When we're compiling the compiler itself we may pull in
                // crates from crates.io, but those crates may depend on other
                // crates also pulled in from crates.io. We want to ideally be
                // able to compile everything without requiring upstream
                // modifications, so in the case that this looks like a
                // `rustc_private` crate (e.g., a compiler crate) and we also have
                // the `-Z force-unstable-if-unmarked` flag present (we're
                // compiling a compiler crate), then let this missing feature
                // annotation slide.
                if feature == sym::rustc_private
                    && issue == NonZero::new(27812)
                    && self.sess.opts.unstable_opts.force_unstable_if_unmarked
                {
                    return EvalResult::Allow;
                }

                if matches!(allow_unstable, AllowUnstable::Yes) {
                    return EvalResult::Allow;
                }

                let suggestion = suggestion_for_allocator_api(self, def_id, span, feature);
                EvalResult::Deny {
                    feature,
                    reason: reason.to_opt_reason(),
                    issue,
                    suggestion,
                    is_soft,
                }
            }
            Some(_) => {
                // Stable APIs are always ok to call and deprecated APIs are
                // handled by the lint emitting logic above.
                EvalResult::Allow
            }
            None => EvalResult::Unmarked,
        }
    }

    /// Evaluates the default-impl stability of an item.
    ///
    /// Returns `EvalResult::Allow` if the item's default implementation is stable, or unstable but the corresponding
    /// `#![feature]` has been provided. Returns `EvalResult::Deny` which describes the offending
    /// unstable feature otherwise.
    pub fn eval_default_body_stability(self, def_id: DefId, span: Span) -> EvalResult {
        let is_staged_api = self.lookup_stability(def_id.krate.as_def_id()).is_some();
        if !is_staged_api {
            return EvalResult::Allow;
        }

        // Only the cross-crate scenario matters when checking unstable APIs
        let cross_crate = !def_id.is_local();
        if !cross_crate {
            return EvalResult::Allow;
        }

        let stability = self.lookup_default_body_stability(def_id);
        debug!(
            "body stability: inspecting def_id={def_id:?} span={span:?} of stability={stability:?}"
        );

        match stability {
            Some(DefaultBodyStability {
                level: hir::StabilityLevel::Unstable { reason, issue, is_soft, .. },
                feature,
            }) => {
                if span.allows_unstable(feature) {
                    debug!("body stability: skipping span={:?} since it is internal", span);
                    return EvalResult::Allow;
                }
                if self.features().enabled(feature) {
                    return EvalResult::Allow;
                }

                EvalResult::Deny {
                    feature,
                    reason: reason.to_opt_reason(),
                    issue,
                    suggestion: None,
                    is_soft,
                }
            }
            Some(_) => {
                // Stable APIs are always ok to call
                EvalResult::Allow
            }
            None => EvalResult::Unmarked,
        }
    }

    /// Checks if an item is stable or error out.
    ///
    /// If the item defined by `def_id` is unstable and the corresponding `#![feature]` does not
    /// exist, emits an error.
    ///
    /// This function will also check if the item is deprecated.
    /// If so, and `id` is not `None`, a deprecated lint attached to `id` will be emitted.
    ///
    /// Returns `true` if item is allowed aka, stable or unstable under an enabled feature.
    pub fn check_stability(
        self,
        def_id: DefId,
        id: Option<HirId>,
        span: Span,
        method_span: Option<Span>,
    ) -> bool {
        self.check_stability_allow_unstable(def_id, id, span, method_span, AllowUnstable::No)
    }

    /// Checks if an item is stable or error out.
    ///
    /// If the item defined by `def_id` is unstable and the corresponding `#![feature]` does not
    /// exist, emits an error.
    ///
    /// This function will also check if the item is deprecated.
    /// If so, and `id` is not `None`, a deprecated lint attached to `id` will be emitted.
    ///
    /// Pass `AllowUnstable::Yes` to `allow_unstable` to force an unstable item to be allowed. Deprecation warnings will be emitted normally.
    ///
    /// Returns `true` if item is allowed aka, stable or unstable under an enabled feature.
    pub fn check_stability_allow_unstable(
        self,
        def_id: DefId,
        id: Option<HirId>,
        span: Span,
        method_span: Option<Span>,
        allow_unstable: AllowUnstable,
    ) -> bool {
        self.check_optional_stability(
            def_id,
            id,
            span,
            method_span,
            allow_unstable,
            |span, def_id| {
                // The API could be uncallable for other reasons, for example when a private module
                // was referenced.
                self.dcx().span_delayed_bug(span, format!("encountered unmarked API: {def_id:?}"));
            },
        )
    }

    /// Like `check_stability`, except that we permit items to have custom behaviour for
    /// missing stability attributes (not necessarily just emit a `bug!`). This is necessary
    /// for default generic parameters, which only have stability attributes if they were
    /// added after the type on which they're defined.
    ///
    /// Returns `true` if item is allowed aka, stable or unstable under an enabled feature.
    pub fn check_optional_stability(
        self,
        def_id: DefId,
        id: Option<HirId>,
        span: Span,
        method_span: Option<Span>,
        allow_unstable: AllowUnstable,
        unmarked: impl FnOnce(Span, DefId),
    ) -> bool {
        let soft_handler = |lint, span, msg: String| {
            self.node_span_lint(lint, id.unwrap_or(hir::CRATE_HIR_ID), span, |lint| {
                lint.primary_message(msg);
            })
        };
        let eval_result =
            self.eval_stability_allow_unstable(def_id, id, span, method_span, allow_unstable);
        let is_allowed = matches!(eval_result, EvalResult::Allow);
        match eval_result {
            EvalResult::Allow => {}
            EvalResult::Deny { feature, reason, issue, suggestion, is_soft } => report_unstable(
                self.sess,
                feature,
                reason,
                issue,
                suggestion,
                is_soft,
                span,
                soft_handler,
                UnstableKind::Regular,
            ),
            EvalResult::Unmarked => unmarked(span, def_id),
        }

        is_allowed
    }

    /// This function is analogous to `check_optional_stability` but with the logic in
    /// `eval_stability_allow_unstable` inlined, and which operating on const stability
    /// instead of regular stability.
    ///
    /// This enforces *syntactical* const stability of const traits. In other words,
    /// it enforces the ability to name `[const]`/`const` traits in trait bounds in various
    /// syntax positions in HIR (including in the trait of an impl header).
    pub fn check_const_stability(self, def_id: DefId, span: Span, const_kw_span: Span) {
        let is_staged_api = self.lookup_stability(def_id.krate.as_def_id()).is_some();
        if !is_staged_api {
            return;
        }

        // Only the cross-crate scenario matters when checking unstable APIs
        let cross_crate = !def_id.is_local();
        if !cross_crate {
            return;
        }

        let stability = self.lookup_const_stability(def_id);
        debug!(
            "stability: \
                inspecting def_id={:?} span={:?} of stability={:?}",
            def_id, span, stability
        );

        match stability {
            Some(ConstStability {
                level: hir::StabilityLevel::Unstable { reason, issue, is_soft, implied_by, .. },
                feature,
                ..
            }) => {
                assert!(!is_soft);

                if span.allows_unstable(feature) {
                    debug!("body stability: skipping span={:?} since it is internal", span);
                    return;
                }
                if self.features().enabled(feature) {
                    return;
                }

                // If this item was previously part of a now-stabilized feature which is still
                // enabled (i.e. the user hasn't removed the attribute for the stabilized feature
                // yet) then allow use of this item.
                if let Some(implied_by) = implied_by
                    && self.features().enabled(implied_by)
                {
                    return;
                }

                report_unstable(
                    self.sess,
                    feature,
                    reason.to_opt_reason(),
                    issue,
                    None,
                    false,
                    span,
                    |_, _, _| {},
                    UnstableKind::Const(const_kw_span),
                );
            }
            Some(_) | None => {}
        }
    }

    pub fn lookup_deprecation(self, id: DefId) -> Option<Deprecation> {
        self.lookup_deprecation_entry(id).map(|depr| depr.attr)
    }
}
```

---
*Generated by AST tracing system*
