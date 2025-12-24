use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Wrapper around a relative [`Utf8Path`].
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RelPath(Utf8Path);
