use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Used to avoid depending on `rustc_middle` in `rustc_attr_parsing`.
/// Always the `TyCtxt`.
pub trait LintEmitter: Copy {
    type Id: Copy;
    #[track_caller]
    fn emit_node_span_lint(
        self,
        lint: &'static Lint,
        hir_id: Self::Id,
        span: impl Into<MultiSpan>,
        decorator: impl for<'a> LintDiagnostic<'a, ()> + DynSend + 'static,
    );
}
