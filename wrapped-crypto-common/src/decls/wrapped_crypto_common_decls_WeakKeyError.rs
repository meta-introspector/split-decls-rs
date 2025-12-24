use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The error type returned when a key is found to be weak.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct WeakKeyError;
