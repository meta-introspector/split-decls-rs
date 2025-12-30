// Generated macro for impl_24 (impl)
macro_rules! Depcrate_bitnessimpl_24 {
() => {
// Module: crate::bitness
// Provides: {"impl_24"}
// Dependencies: {}
impl Display for Bitness { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match * self { Bitness :: Unknown => write ! (f , "unknown bitness") , Bitness :: X32 => write ! (f , "32-bit") , Bitness :: X64 => write ! (f , "64-bit") , } } }
};
}
