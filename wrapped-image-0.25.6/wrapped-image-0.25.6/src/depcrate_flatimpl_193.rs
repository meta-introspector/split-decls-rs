// Generated macro for impl_193 (impl)
macro_rules! Depcrate_flatimpl_193 {
() => {
// Module: crate::flat
// Provides: {"impl_193"}
// Dependencies: {}
impl Dim { fn stride (self) -> usize { self . 0 } # [doc = " Length of this dimension in memory."] fn checked_len (self) -> Option < usize > { self . 0 . checked_mul (self . 1) } fn len (self) -> usize { self . 0 * self . 1 } }
};
}
