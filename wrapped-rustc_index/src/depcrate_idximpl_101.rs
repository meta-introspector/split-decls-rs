// Generated macro for impl_101 (impl)
macro_rules! Depcrate_idximpl_101 {
() => {
// Module: crate::idx
// Provides: {"impl_101"}
// Dependencies: {}
impl Idx for u32 { # [inline] fn new (idx : usize) -> Self { assert ! (idx <= u32 :: MAX as usize) ; idx as u32 } # [inline] fn index (self) -> usize { self as usize } }
};
}
