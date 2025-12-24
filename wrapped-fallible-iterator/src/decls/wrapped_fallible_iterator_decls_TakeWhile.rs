use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which yields elements based on a predicate.
#[derive(Clone, Debug)]
pub struct TakeWhile<I, P> {
    it: I,
    flag: bool,
    predicate: P,
}
