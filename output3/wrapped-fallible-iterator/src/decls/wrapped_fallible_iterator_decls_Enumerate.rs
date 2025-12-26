use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that yields the iteration count as well as the values of the
/// underlying iterator.
#[derive(Clone, Debug)]
pub struct Enumerate<I> {
    it: I,
    n: usize,
}
