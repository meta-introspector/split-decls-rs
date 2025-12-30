// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_runtime_ivarimpl_1051 {
() => {
// Module: crate::runtime::ivar
// Provides: {"impl_1051"}
// Dependencies: {}
impl fmt :: Debug for Ivar { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ivar") . field ("name" , & self . name ()) . field ("offset" , & self . offset ()) . field ("type_encoding" , & self . type_encoding ()) . finish_non_exhaustive () } }
};
}
