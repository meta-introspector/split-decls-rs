use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait which parser rules must implement.
///
/// This trait is set up so that any struct that implements all of its required traits will
/// automatically implement this trait as well.
///
/// This is essentially a [trait alias](https://github.com/rust-lang/rfcs/pull/1733). When trait
/// aliases are implemented, this may be replaced by one.
pub trait RuleType: Copy + Debug + Eq + Hash + Ord {}
