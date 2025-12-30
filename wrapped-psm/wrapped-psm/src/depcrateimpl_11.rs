// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl StackDirection { # [doc = " Obtain the stack growth direction."] # [cfg (asm)] pub fn new () -> StackDirection { const ASC : u8 = StackDirection :: Ascending as u8 ; const DSC : u8 = StackDirection :: Descending as u8 ; unsafe { match rust_psm_stack_direction () { ASC => StackDirection :: Ascending , DSC => StackDirection :: Descending , _ => :: core :: hint :: unreachable_unchecked () , } } } }
};
}
