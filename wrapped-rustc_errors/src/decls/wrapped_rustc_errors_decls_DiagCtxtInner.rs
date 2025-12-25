use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This inner struct exists to keep it all behind a single lock;
/// this is done to prevent possible deadlocks in a multi-threaded compiler,
/// as well as inconsistent state observation.
struct DiagCtxtInner {
    flags: DiagCtxtFlags,
    registry: Registry,
    /// The error guarantees from all emitted errors. The length gives the error count.
    err_guars: Vec<ErrorGuaranteed>,
    /// The error guarantee from all emitted lint errors. The length gives the
    /// lint error count.
    lint_err_guars: Vec<ErrorGuaranteed>,
    /// The delayed bugs and their error guarantees.
    delayed_bugs: Vec<(DelayedDiagInner, ErrorGuaranteed)>,
    /// The error count shown to the user at the end.
    deduplicated_err_count: usize,
    /// The warning count shown to the user at the end.
    deduplicated_warn_count: usize,
    emitter: Box<DynEmitter>,
    /// Must we produce a diagnostic to justify the use of the expensive
    /// `trimmed_def_paths` function? Backtrace is the location of the call.
    must_produce_diag: Option<Backtrace>,
    /// Has this diagnostic context printed any diagnostics? (I.e. has
    /// `self.emitter.emit_diagnostic()` been called?
    has_printed: bool,
    /// This flag indicates that an expected diagnostic was emitted and suppressed.
    /// This is used for the `must_produce_diag` check.
    suppressed_expected_diag: bool,
    /// This set contains the code of all emitted diagnostics to avoid
    /// emitting the same diagnostic with extended help (`--teach`) twice, which
    /// would be unnecessary repetition.
    taught_diagnostics: FxHashSet<ErrCode>,
    /// Used to suggest rustc --explain `<error code>`
    emitted_diagnostic_codes: FxIndexSet<ErrCode>,
    /// This set contains a hash of every diagnostic that has been emitted by
    /// this `DiagCtxt`. These hashes is used to avoid emitting the same error
    /// twice.
    emitted_diagnostics: FxHashSet<Hash128>,
    /// Stashed diagnostics emitted in one stage of the compiler that may be
    /// stolen and emitted/cancelled by other stages (e.g. to improve them and
    /// add more information). All stashed diagnostics must be emitted with
    /// `emit_stashed_diagnostics` by the time the `DiagCtxtInner` is dropped,
    /// otherwise an assertion failure will occur.
    stashed_diagnostics: FxIndexMap<
        StashKey,
        FxIndexMap<Span, (DiagInner, Option<ErrorGuaranteed>)>,
    >,
    future_breakage_diagnostics: Vec<DiagInner>,
    /// expected diagnostic will have the level `Expect` which additionally
    /// carries the [`LintExpectationId`] of the expectation that can be
    /// marked as fulfilled. This is a collection of all [`LintExpectationId`]s
    /// that have been marked as fulfilled this way.
    ///
    /// Emitting expectations after having stolen this field can happen. In particular, an
    /// `#[expect(warnings)]` can easily make the `UNFULFILLED_LINT_EXPECTATIONS` lint expect
    /// itself. To avoid needless complexity in this corner case, we tolerate failing to track
    /// those expectations.
    ///
    /// [RFC-2383]: https://rust-lang.github.io/rfcs/2383-lint-reasons.html
    fulfilled_expectations: FxIndexSet<LintExpectationId>,
    /// The file where the ICE information is stored. This allows delayed_span_bug backtraces to be
    /// stored along side the main panic backtrace.
    ice_file: Option<PathBuf>,
}
