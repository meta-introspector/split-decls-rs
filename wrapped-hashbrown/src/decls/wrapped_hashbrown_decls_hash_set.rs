use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod hash_set {
    //! A hash set implemented as a `HashMap` where the value is `()`.
    pub use crate::set::*;
    #[cfg(feature = "rayon")]
    /// [rayon]-based parallel iterator types for hash sets.
    /// You will rarely need to interact with it directly unless you have need
    /// to name one of the iterator types.
    ///
    /// [rayon]: https://docs.rs/rayon/1.0/rayon
    pub mod rayon {
        pub use crate::external_trait_impls::rayon::set::*;
    }
}
