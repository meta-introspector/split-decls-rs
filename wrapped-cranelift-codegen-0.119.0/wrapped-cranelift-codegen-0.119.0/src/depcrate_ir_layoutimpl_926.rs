// Generated macro for impl_926 (impl)
macro_rules! Depcrate_ir_layoutimpl_926 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_926"}
// Dependencies: {}
impl core :: hash :: Hash for InstNode { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . block . hash (state) ; self . prev . hash (state) ; self . next . hash (state) ; } }
};
}
