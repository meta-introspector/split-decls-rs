// Generated macro for impl_21 (impl)
macro_rules! Depcrate_literalsimpl_21 {
() => {
// Module: crate::literals
// Provides: {"impl_21"}
// Dependencies: {}
impl Lit { # [doc = " Returns a new complete literal with the bytes given."] pub fn new (bytes : Vec < u8 >) -> Lit { Lit { v : bytes , cut : false } } # [doc = " Returns a new complete empty literal."] pub fn empty () -> Lit { Lit { v : vec ! [] , cut : false } } # [doc = " Returns true if this literal was \"cut.\""] pub fn is_cut (& self) -> bool { self . cut } # [doc = " Cuts this literal."] pub fn cut (& mut self) { self . cut = true ; } }
};
}
