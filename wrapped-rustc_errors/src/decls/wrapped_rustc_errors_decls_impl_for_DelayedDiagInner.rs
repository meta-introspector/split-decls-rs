use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DelayedDiagInner {
    fn with_backtrace(diagnostic: DiagInner, backtrace: Backtrace) -> Self {
        DelayedDiagInner {
            inner: diagnostic,
            note: backtrace,
        }
    }
    fn decorate(self, dcx: &DiagCtxtInner) -> DiagInner {
        let mut diag = self.inner;
        let msg = match self.note.status() {
            BacktraceStatus::Captured => {
                crate::fluent_generated::errors_delayed_at_with_newline
            }
            _ => crate::fluent_generated::errors_delayed_at_without_newline,
        };
        diag.arg("emitted_at", diag.emitted_at.clone());
        diag.arg("note", self.note);
        let msg = dcx.eagerly_translate_for_subdiag(&diag, msg);
        diag.sub(Note, msg, diag.span.primary_span().unwrap_or(DUMMY_SP).into());
        diag
    }
}
