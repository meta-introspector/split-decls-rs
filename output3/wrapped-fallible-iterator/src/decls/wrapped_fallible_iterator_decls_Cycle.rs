use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which cycles another endlessly.
#[derive(Clone, Debug)]
pub struct Cycle<I> {
    it: I,
    cur: I,
}
