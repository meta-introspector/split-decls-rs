use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Runs a closure and catches unwinds triggered by fatal errors.
///
/// The compiler currently unwinds with a special sentinel value to abort
/// compilation on fatal errors. This function catches that sentinel and turns
/// the panic into a `Result` instead.
pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, FatalError> {
    catch_unwind(panic::AssertUnwindSafe(f))
        .map_err(|value| {
            if value.is::<rustc_errors::FatalErrorMarker>() {
                FatalError
            } else {
                panic::resume_unwind(value);
            }
        })
}
