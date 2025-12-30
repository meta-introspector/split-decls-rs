// Generated macro for impl_186 (impl)
macro_rules! Depcrate_dfa_accelimpl_186 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_186"}
// Dependencies: {}
impl < A : AsRef < [AccelTy] > > core :: fmt :: Debug for Accels < A > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Accels(") ? ; let mut list = f . debug_list () ; for a in self . iter () { list . entry (& a) ; } list . finish () ? ; write ! (f , ")") } }
};
}
