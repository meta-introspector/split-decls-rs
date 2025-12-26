use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> FoldWhile<T> {
    /// Return the value in the continue or done.
    pub fn into_inner(self) -> T {
        match self {
            Self::Continue(x) | Self::Done(x) => x,
        }
    }
    /// Return true if `self` is `Done`, false if it is `Continue`.
    pub fn is_done(&self) -> bool {
        match *self {
            Self::Continue(_) => false,
            Self::Done(_) => true,
        }
    }
}
