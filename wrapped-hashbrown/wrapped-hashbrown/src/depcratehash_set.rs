// Generated macro for hash_set (module)
macro_rules! Depcratehash_set {
() => {
// Module: crate
// Provides: {"hash_set"}
// Dependencies: {}
pub mod hash_set { # ! [doc = " A hash set implemented as a `HashMap` where the value is `()`."] pub use crate :: set :: * ; # [cfg (feature = "rayon")] # [doc = " [rayon]-based parallel iterator types for hash sets."] # [doc = " You will rarely need to interact with it directly unless you have need"] # [doc = " to name one of the iterator types."] # [doc = ""] # [doc = " [rayon]: https://docs.rs/rayon/1.0/rayon"] pub mod rayon { pub use crate :: external_trait_impls :: rayon :: set :: * ; } }
};
}
