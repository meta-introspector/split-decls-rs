// Generated macro for impl_203 (impl)
macro_rules! Depcrate_limbimpl_203 {
() => {
// Module: crate::limb
// Provides: {"impl_203"}
// Dependencies: {}
impl fmt :: UpperHex for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{:0width$X}" , & self . 0 , width = Self :: BYTES * 2) } }
};
}
