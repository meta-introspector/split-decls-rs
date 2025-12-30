// Generated macro for impl_228 (impl)
macro_rules! Depcrate_animationimpl_228 {
() => {
// Module: crate::animation
// Provides: {"impl_228"}
// Dependencies: {}
impl From < Delay > for Duration { fn from (delay : Delay) -> Self { let ratio = delay . into_ratio () ; let ms = ratio . to_integer () ; let rest = ratio . numer % ratio . denom ; let nanos = (u64 :: from (rest) * 1_000_000) / u64 :: from (ratio . denom) ; Duration :: from_millis (ms . into ()) + Duration :: from_nanos (nanos) } }
};
}
