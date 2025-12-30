// Generated macro for impl_11 (impl)
macro_rules! Depcrate_iterimpl_11 {
() => {
// Module: crate::iter
// Provides: {"impl_11"}
// Dependencies: {}
impl Iterator for Bytes < '_ > { type Item = u8 ; # [inline] fn next (& mut self) -> Option < u8 > { if self . cursor < self . end { unsafe { let b = * self . cursor ; self . bump () ; Some (b) } } else { None } } }
};
}
