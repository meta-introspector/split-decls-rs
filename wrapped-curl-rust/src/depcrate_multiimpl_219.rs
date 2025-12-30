// Generated macro for impl_219 (impl)
macro_rules! Depcrate_multiimpl_219 {
() => {
// Module: crate::multi
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Message < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Message") . field ("ptr" , & self . ptr) . finish () } }
};
}
