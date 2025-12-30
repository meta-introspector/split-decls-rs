// Generated macro for impl_201 (impl)
macro_rules! Depcrate_limbimpl_201 {
() => {
// Module: crate::limb
// Provides: {"impl_201"}
// Dependencies: {}
impl fmt :: Binary for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "0b") ? ; } write ! (f , "{:0width$b}" , & self . 0 , width = Self :: BITS as usize) } }
};
}
