// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Neg for Round { type Output = Round ; # [inline] fn neg (self) -> Round { match self { Round :: TowardPositive => Round :: TowardNegative , Round :: TowardNegative => Round :: TowardPositive , Round :: NearestTiesToEven | Round :: TowardZero | Round :: NearestTiesToAway => self , } } }
};
}
