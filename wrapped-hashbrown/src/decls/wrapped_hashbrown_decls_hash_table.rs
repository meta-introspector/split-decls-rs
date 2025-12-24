use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod hash_table {
    //! A hash table implemented with quadratic probing and SIMD lookup.
    pub use crate::table::*;
    #[cfg(feature = "rayon")]
    /// [rayon]-based parallel iterator types for hash tables.
    /// You will rarely need to interact with it directly unless you have need
    /// to name one of the iterator types.
    ///
    /// [rayon]: https://docs.rs/rayon/1.0/rayon
    pub mod rayon {
        pub use crate::external_trait_impls::rayon::table::*;
    }
}
