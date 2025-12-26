use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which yields a limited number of elements from the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct Take<I> {
    it: I,
    remaining: usize,
}
