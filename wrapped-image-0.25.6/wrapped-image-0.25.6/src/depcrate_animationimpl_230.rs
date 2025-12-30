// Generated macro for impl_230 (impl)
macro_rules! Depcrate_animationimpl_230 {
() => {
// Module: crate::animation
// Provides: {"impl_230"}
// Dependencies: {}
impl Ratio { # [inline] pub (crate) fn new (numerator : u32 , denominator : u32) -> Self { assert_ne ! (denominator , 0) ; Self { numer : numerator , denom : denominator , } } # [inline] pub (crate) fn to_integer (self) -> u32 { self . numer / self . denom } }
};
}
