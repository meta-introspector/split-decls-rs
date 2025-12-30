// Generated macro for impl_381 (impl)
macro_rules! Depcrate_expand_typetreeimpl_381 {
() => {
// Module: crate::expand::typetree
// Provides: {"impl_381"}
// Dependencies: {}
impl Type { pub fn add_offset (self , add : isize) -> Self { let offset = match self . offset { - 1 => add , x => add + x , } ; Self { size : self . size , kind : self . kind , child : self . child , offset } } }
};
}
