// Generated macro for c_hw (function)
macro_rules! Depcrate_float_divc_hw {
() => {
// Module: crate::float::div
// Provides: {"c_hw"}
// Dependencies: {}
# [doc = " The value of `C` adjusted to half width."] # [doc = ""] # [doc = " C is (3/4 + 1/sqrt(2)) - 1 truncated to W0 fractional bits as UQ0.HW with W0 being either"] # [doc = " 16 or 32 and W0 <= HW. That is, C is the aforementioned 3/4 + 1/sqrt(2) constant (from"] # [doc = " which b/2 is subtracted to obtain x0) wrapped to [0, 1) range."] fn c_hw < F : Float > () -> HalfRep < F > where F :: Int : DInt , u128 : CastInto < HalfRep < F > > , { const C_U128 : u128 = 0x7504f333f9de6108b2fb1366eaa6a542 ; const { C_U128 >> (u128 :: BITS - < HalfRep < F > > :: BITS) } . cast () }
};
}
