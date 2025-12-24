use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Traits helpful for using certain `Itertools` methods in generic contexts.
pub mod traits {
    pub use crate::iter_index::IteratorIndex;
    pub use crate::tuple_impl::HomogeneousTuple;
}
