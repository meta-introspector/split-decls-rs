// Generated macro for impl_2703 (impl)
macro_rules! Depcrate_isaimpl_2703 {
() => {
// Module: crate::isa
// Provides: {"impl_2703"}
// Dependencies: {}
impl Debug for & dyn TargetIsa { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "TargetIsa {{ triple: {:?}, pointer_width: {:?}}}" , self . triple () , self . pointer_width ()) } }
};
}
