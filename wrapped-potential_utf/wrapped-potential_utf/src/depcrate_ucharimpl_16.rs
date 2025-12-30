// Generated macro for impl_16 (impl)
macro_rules! Depcrate_ucharimpl_16 {
() => {
// Module: crate::uchar
// Provides: {"impl_16"}
// Dependencies: {}
impl From < PotentialCodePoint > for u32 { fn from (x : PotentialCodePoint) -> Self { let [a0 , a1 , a2] = x . 0 ; u32 :: from_le_bytes ([a0 , a1 , a2 , 0]) } }
};
}
