use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Copy, Clone, Default)]
pub struct DiagCtxtFlags {
    /// If false, warning-level lints are suppressed.
    /// (rustc: see `--allow warnings` and `--cap-lints`)
    pub can_emit_warnings: bool,
    /// If Some, the Nth error-level diagnostic is upgraded to bug-level.
    /// (rustc: see `-Z treat-err-as-bug`)
    pub treat_err_as_bug: Option<NonZero<usize>>,
    /// Eagerly emit delayed bugs as errors, so that the compiler debugger may
    /// see all of the errors being emitted at once.
    pub eagerly_emit_delayed_bugs: bool,
    /// Show macro backtraces.
    /// (rustc: see `-Z macro-backtrace`)
    pub macro_backtrace: bool,
    /// If true, identical diagnostics are reported only once.
    pub deduplicate_diagnostics: bool,
    /// Track where errors are created. Enabled with `-Ztrack-diagnostics`.
    pub track_diagnostics: bool,
}
