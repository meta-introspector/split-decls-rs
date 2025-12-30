// Generated macro for impl_38 (impl)
macro_rules! Depcrate_powimpl_38 {
() => {
// Module: crate::pow
// Provides: {"impl_38"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl < T : Float > Pow < Complex < T > > for Complex < T > { type Output = Complex < T > ; # [inline] fn pow (self , exp : Complex < T >) -> Self :: Output { self . powc (exp) } }
};
}
