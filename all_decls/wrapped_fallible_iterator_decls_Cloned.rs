use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which clones the elements of the underlying iterator.
#[derive(Clone, Debug)]
pub struct Cloned<I>(I);
