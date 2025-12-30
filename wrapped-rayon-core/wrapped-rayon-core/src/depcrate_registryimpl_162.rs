// Generated macro for impl_162 (impl)
macro_rules! Depcrate_registryimpl_162 {
() => {
// Module: crate::registry
// Provides: {"impl_162"}
// Dependencies: {}
impl fmt :: Debug for ThreadBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ThreadBuilder") . field ("pool" , & self . registry . id ()) . field ("index" , & self . index) . field ("name" , & self . name) . field ("stack_size" , & self . stack_size) . finish () } }
};
}
