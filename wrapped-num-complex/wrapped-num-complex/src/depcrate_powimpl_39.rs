// Generated macro for impl_39 (impl)
macro_rules! Depcrate_powimpl_39 {
() => {
// Module: crate::pow
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl < 'b , T : Float > Pow < & 'b Complex < T > > for Complex < T > { type Output = Complex < T > ; # [inline] fn pow (self , & exp : & 'b Complex < T >) -> Self :: Output { self . powc (exp) } }
};
}
