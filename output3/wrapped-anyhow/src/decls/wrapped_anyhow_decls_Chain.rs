use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Iterator of a chain of source errors.
///
/// This type is the iterator returned by [`Error::chain`].
///
/// # Example
///
/// ```
/// use anyhow::Error;
/// use std::io;
///
/// pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {
///     for cause in error.chain() {
///         if let Some(io_error) = cause.downcast_ref::<io::Error>() {
///             return Some(io_error.kind());
///         }
///     }
///     None
/// }
/// ```
#[cfg(any(feature = "std", not(anyhow_no_core_error)))]
#[derive(Clone)]
pub struct Chain<'a> {
    state: crate::chain::ChainState<'a>,
}
