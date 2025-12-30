// Generated macro for impl_71 (impl)
macro_rules! Depcrate_eqimpl_71 {
() => {
// Module: crate::eq
// Provides: {"impl_71"}
// Dependencies: {}
impl ApproxEq for f64 { type Margin = F64Margin ; fn approx_eq < M : Into < Self :: Margin > > (self , other : f64 , margin : M) -> bool { let margin = margin . into () ; self == other || { let eps = f64abs (self - other) ; (eps <= margin . epsilon) || { let diff : i64 = self . ulps (& other) ; saturating_abs_i64 ! (diff) <= margin . ulps } } } }
};
}
