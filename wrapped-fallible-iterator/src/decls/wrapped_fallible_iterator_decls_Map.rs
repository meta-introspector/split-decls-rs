use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which applies a fallible transform to the elements of the
/// underlying iterator.
#[derive(Clone)]
pub struct Map<T, F> {
    it: T,
    f: F,
}
