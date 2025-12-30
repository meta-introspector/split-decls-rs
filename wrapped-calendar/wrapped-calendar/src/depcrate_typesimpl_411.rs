// Generated macro for impl_411 (impl)
macro_rules! Depcrate_typesimpl_411 {
() => {
// Module: crate::types
// Provides: {"impl_411"}
// Dependencies: {}
impl From < RataDie > for Weekday { fn from (value : RataDie) -> Self { use Weekday :: * ; match (value - SUNDAY) . rem_euclid (7) { 0 => Sunday , 1 => Monday , 2 => Tuesday , 3 => Wednesday , 4 => Thursday , 5 => Friday , 6 => Saturday , _ => unreachable ! () , } } }
};
}
