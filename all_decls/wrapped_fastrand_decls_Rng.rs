use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A random number generator.
#[derive(Debug, PartialEq, Eq)]
pub struct Rng(u64);
