// Generated macro for impl_268 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_268 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_268"}
// Dependencies: {}
impl < Z : TimeZone > From < Range < DateTime < Z > > > for RangedDateTime < DateTime < Z > > { fn from (range : Range < DateTime < Z > >) -> Self { Self (range . start , range . end) } }
};
}
