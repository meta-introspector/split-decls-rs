// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "libm"))] impl < T : Float + FloatConst > Complex < T > { # [doc = " Computes `2^(self)`."] # [inline] pub fn exp2 (self) -> Self { Self :: from_polar (self . re . exp2 () , self . im * T :: LN_2 ()) } # [doc = " Computes the principal value of log base 2 of `self`."] # [inline] pub fn log2 (self) -> Self { Self :: ln (self) / T :: LN_2 () } # [doc = " Computes the principal value of log base 10 of `self`."] # [inline] pub fn log10 (self) -> Self { Self :: ln (self) / T :: LN_10 () } }
};
}
