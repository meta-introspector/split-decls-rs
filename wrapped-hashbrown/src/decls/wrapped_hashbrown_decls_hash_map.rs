use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    pub use crate::map::*;
    #[cfg(feature = "rustc-internal-api")]
    pub use crate::rustc_entry::*;
    #[cfg(feature = "rayon")]
    /// [rayon]-based parallel iterator types for hash maps.
    /// You will rarely need to interact with it directly unless you have need
    /// to name one of the iterator types.
    ///
    /// [rayon]: https://docs.rs/rayon/1.0/rayon
    pub mod rayon {
        pub use crate::external_trait_impls::rayon::map::*;
    }
}
