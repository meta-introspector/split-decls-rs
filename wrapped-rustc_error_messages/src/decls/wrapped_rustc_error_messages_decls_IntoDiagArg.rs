use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts a value of a type into a `DiagArg` (typically a field of an `Diag` struct).
/// Implemented as a custom trait rather than `From` so that it is implemented on the type being
/// converted rather than on `DiagArgValue`, which enables types from other `rustc_*` crates to
/// implement this.
pub trait IntoDiagArg {
    /// Convert `Self` into a `DiagArgValue` suitable for rendering in a diagnostic.
    ///
    /// It takes a `path` where "long values" could be written to, if the `DiagArgValue` is too big
    /// for displaying on the terminal. This path comes from the `Diag` itself. When rendering
    /// values that come from `TyCtxt`, like `Ty<'_>`, they can use `TyCtxt::short_string`. If a
    /// value has no shortening logic that could be used, the argument can be safely ignored.
    fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue;
}
