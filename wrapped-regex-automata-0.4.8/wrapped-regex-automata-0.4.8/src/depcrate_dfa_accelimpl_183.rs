// Generated macro for impl_183 (impl)
macro_rules! Depcrate_dfa_accelimpl_183 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_183"}
// Dependencies: {}
impl < A : AsRef < [AccelTy] > > core :: fmt :: Debug for Accels < A > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Accels(") ? ; let mut list = f . debug_list () ; for a in self . iter () { list . entry (& a) ; } list . finish () ? ; write ! (f , ")") } }
};
}
