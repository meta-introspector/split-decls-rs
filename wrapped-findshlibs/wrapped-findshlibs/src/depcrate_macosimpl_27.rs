// Generated macro for impl_27 (impl)
macro_rules! Depcrate_macosimpl_27 {
() => {
// Module: crate::macos
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > fmt :: Debug for SharedLibrary < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("SharedLibrary") . field ("name" , & self . name ()) . field ("id" , & self . id ()) . finish () } }
};
}
