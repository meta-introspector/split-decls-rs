// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_runtime_methodimpl_1121 {
() => {
// Module: crate::runtime::method
// Provides: {"impl_1121"}
// Dependencies: {}
impl fmt :: Debug for Method { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Method") . field ("name" , & self . name ()) . field ("types" , & self . types ()) . field ("implementation" , & self . implementation ()) . finish_non_exhaustive () } }
};
}
