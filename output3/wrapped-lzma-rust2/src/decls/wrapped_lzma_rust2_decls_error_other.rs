use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
#[inline(always)]
fn error_other(msg: &'static str) -> Error {
    Error::Other(msg)
}
