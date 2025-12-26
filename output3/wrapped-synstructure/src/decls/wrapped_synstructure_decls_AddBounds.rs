use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Changes how bounds are added
#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AddBounds {
    /// Add for fields and generics
    Both,
    /// Fields only
    Fields,
    /// Generics only
    Generics,
    /// None
    None,
    #[doc(hidden)]
    __Nonexhaustive,
}
