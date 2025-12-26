use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A normal (non-fallible) iterator which wraps a fallible iterator.
#[derive(Clone, Debug)]
pub struct Iterator<I>(I);
