// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl PartialEq for FixedBitSet { fn eq (& self , other : & Self) -> bool { self . length == other . length && self . as_simd_slice () . eq (other . as_simd_slice ()) } }
};
}
