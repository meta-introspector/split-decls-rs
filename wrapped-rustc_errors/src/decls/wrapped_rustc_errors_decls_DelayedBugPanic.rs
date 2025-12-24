use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Signifies that the compiler died due to a delayed bug rather than a failed
/// assertion, etc.
pub struct DelayedBugPanic;
