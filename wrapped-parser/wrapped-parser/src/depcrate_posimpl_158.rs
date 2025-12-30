// Generated macro for impl_158 (impl)
macro_rules! Depcrate_posimpl_158 {
() => {
// Module: crate::pos
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for Positioned < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . node . fmt (f) } }
};
}
