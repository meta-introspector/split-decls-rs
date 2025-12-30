// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
impl Ord for FixedBitSet { fn cmp (& self , other : & Self) -> Ordering { self . length . cmp (& other . length) . then_with (| | self . as_simd_slice () . cmp (other . as_simd_slice ())) } }
};
}
