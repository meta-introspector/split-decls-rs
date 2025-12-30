// Generated macro for impl_281 (impl)
macro_rules! Depcrate_hirimpl_281 {
() => {
// Module: crate::hir
// Provides: {"impl_281"}
// Dependencies: {}
impl DotDotPos { # [doc = " Panics if n >= u32::MAX."] pub fn new (n : Option < usize >) -> Self { match n { Some (n) => { assert ! (n < u32 :: MAX as usize) ; Self (n as u32) } None => Self (u32 :: MAX) , } } pub fn as_opt_usize (& self) -> Option < usize > { if self . 0 == u32 :: MAX { None } else { Some (self . 0 as usize) } } }
};
}
