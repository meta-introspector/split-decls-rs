use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait implemented for `tracing` types that can be converted to a `log`
/// equivalent.
pub trait AsLog: crate::sealed::Sealed {
    /// The `log` type that this type can be converted into.
    type Log;
    /// Returns the `log` equivalent of `self`.
    fn as_log(&self) -> Self::Log;
}
