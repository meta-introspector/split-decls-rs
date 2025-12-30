// Generated macro for impl_17 (impl)
macro_rules! Depcrate_macosimpl_17 {
() => {
// Module: crate::macos
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Segment < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Segment") . field ("name" , & self . name ()) . field ("is_code" , & self . is_code ()) . finish () } }
};
}
