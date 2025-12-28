macro_rules! hash_map {
    () => {
        pub mod hash_map { # ! [doc = " A hash map implemented with quadratic probing and SIMD lookup."] pub use crate :: map :: * ; # [cfg (feature = "rustc-internal-api")] pub use crate :: rustc_entry :: * ; # [cfg (feature = "rayon")] # [doc = " [rayon]-based parallel iterator types for hash maps."] # [doc = " You will rarely need to interact with it directly unless you have need"] # [doc = " to name one of the iterator types."] # [doc = ""] # [doc = " [rayon]: https://docs.rs/rayon/1.0/rayon"] pub mod rayon { pub use crate :: external_trait_impls :: rayon :: map :: * ; } }
    };
}

hash_map!()