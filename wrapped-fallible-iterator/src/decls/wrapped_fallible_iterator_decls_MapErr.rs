use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which applies a transform to the errors of the underlying
/// iterator.
#[derive(Clone, Debug)]
pub struct MapErr<I, F> {
    it: I,
    f: F,
}
