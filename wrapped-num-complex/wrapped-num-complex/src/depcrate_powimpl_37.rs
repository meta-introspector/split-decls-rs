// Generated macro for impl_37 (impl)
macro_rules! Depcrate_powimpl_37 {
() => {
// Module: crate::pow
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl < 'a , 'b , T : Float > Pow < & 'b Complex < T > > for & 'a Complex < T > { type Output = Complex < T > ; # [inline] fn pow (self , & exp : & 'b Complex < T >) -> Self :: Output { self . powc (exp) } }
};
}
