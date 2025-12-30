// Generated macro for impl_13 (impl)
macro_rules! Depcrate_cfiimpl_13 {
() => {
// Module: crate::cfi
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for FontInfo { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("FontInfo") . field ("dwFontSize" , & self . size ()) . field ("nFont" , & self . index ()) . finish () } }
};
}
