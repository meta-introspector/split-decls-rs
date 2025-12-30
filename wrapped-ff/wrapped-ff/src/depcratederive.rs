// Generated macro for derive (module)
macro_rules! Depcratederive {
() => {
// Module: crate
// Provides: {"derive"}
// Dependencies: {}
# [doc = " Functions and re-exported crates used by the [`PrimeField`] derive macro."] # [cfg (feature = "derive")] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub mod derive { pub use crate :: arith_impl :: * ; pub use { byteorder , rand_core , subtle } ; # [cfg (feature = "bits")] pub use bitvec ; }
};
}
