// Generated macro for impl_59 (impl)
macro_rules! Depcrate_eqimpl_59 {
() => {
// Module: crate::eq
// Provides: {"impl_59"}
// Dependencies: {}
impl ApproxEq for f32 { type Margin = F32Margin ; fn approx_eq < M : Into < Self :: Margin > > (self , other : f32 , margin : M) -> bool { let margin = margin . into () ; self == other || { let eps = f32abs (self - other) ; (eps <= margin . epsilon) || { let diff : i32 = self . ulps (& other) ; saturating_abs_i32 ! (diff) <= margin . ulps } } } }
};
}
