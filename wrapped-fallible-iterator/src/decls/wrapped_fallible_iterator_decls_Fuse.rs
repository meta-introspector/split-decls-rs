use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that yields `Ok(None)` forever after the underlying iterator
/// yields `Ok(None)` once.
#[derive(Clone, Debug)]
pub struct Fuse<I> {
    it: I,
    done: bool,
}
