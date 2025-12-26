use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Simplified version of `FluentArg` that can implement `Encodable` and `Decodable`. Collection of
/// `DiagArg` are converted to `FluentArgs` (consuming the collection) at the start of diagnostic
/// emission.
pub type DiagArg<'iter> = (&'iter DiagArgName, &'iter DiagArgValue);
