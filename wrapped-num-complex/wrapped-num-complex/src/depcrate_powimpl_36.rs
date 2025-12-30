// Generated macro for impl_36 (impl)
macro_rules! Depcrate_powimpl_36 {
() => {
// Module: crate::pow
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl < 'a , T : Float > Pow < Complex < T > > for & 'a Complex < T > { type Output = Complex < T > ; # [inline] fn pow (self , exp : Complex < T >) -> Self :: Output { self . powc (exp) } }
};
}
