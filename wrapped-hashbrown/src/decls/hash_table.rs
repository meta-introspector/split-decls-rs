macro_rules! hash_table {
    () => {
        pub mod hash_table { # ! [doc = " A hash table implemented with quadratic probing and SIMD lookup."] pub use crate :: table :: * ; # [cfg (feature = "rayon")] # [doc = " [rayon]-based parallel iterator types for hash tables."] # [doc = " You will rarely need to interact with it directly unless you have need"] # [doc = " to name one of the iterator types."] # [doc = ""] # [doc = " [rayon]: https://docs.rs/rayon/1.0/rayon"] pub mod rayon { pub use crate :: external_trait_impls :: rayon :: table :: * ; } }
    };
}

hash_table!()