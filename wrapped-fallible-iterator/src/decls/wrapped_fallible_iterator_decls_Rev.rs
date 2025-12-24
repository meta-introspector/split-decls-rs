use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which yields elements of the underlying iterator in reverse
/// order.
#[derive(Clone, Debug)]
pub struct Rev<I>(I);
