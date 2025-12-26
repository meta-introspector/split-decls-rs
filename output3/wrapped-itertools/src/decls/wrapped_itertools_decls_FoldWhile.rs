use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enum used for controlling the execution of `fold_while`.
///
/// See [`.fold_while()`](Itertools::fold_while) for more information.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FoldWhile<T> {
    /// Continue folding with this value
    Continue(T),
    /// Fold is complete and will return this value
    Done(T),
}
