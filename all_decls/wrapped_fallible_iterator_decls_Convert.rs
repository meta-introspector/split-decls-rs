use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A fallible iterator that wraps a normal iterator over `Result`s.
#[derive(Clone, Debug)]
pub struct Convert<I>(I);
