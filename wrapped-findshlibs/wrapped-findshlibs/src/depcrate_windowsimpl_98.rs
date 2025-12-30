// Generated macro for impl_98 (impl)
macro_rules! Depcrate_windowsimpl_98 {
() => {
// Module: crate::windows
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a > fmt :: Debug for SharedLibrary < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("SharedLibrary") . field ("module_base" , & self . module_base ()) . field ("name" , & self . name ()) . field ("debug_name" , & self . debug_name ()) . field ("id" , & self . id ()) . field ("debug_id" , & self . debug_id ()) . finish () } }
};
}
