// Generated macro for FxBuildHasher (struct)
macro_rules! DepcrateFxBuildHasher {
() => {
// Module: crate
// Provides: {"FxBuildHasher"}
// Dependencies: {}
# [doc = " An implementation of [`BuildHasher`] that produces [`FxHasher`]s."] # [doc = ""] # [doc = " ```"] # [doc = " use std::hash::BuildHasher;"] # [doc = " use rustc_hash::FxBuildHasher;"] # [doc = " assert_ne!(FxBuildHasher.hash_one(1), FxBuildHasher.hash_one(2));"] # [doc = " ```"] # [derive (Copy , Clone , Default)] pub struct FxBuildHasher ;
};
}
