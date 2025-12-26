use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator which yields the elements of one iterator followed by another.
#[derive(Clone, Debug)]
pub struct Chain<T, U> {
    front: T,
    back: U,
    state: ChainState,
}
