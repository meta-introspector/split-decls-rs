// Generated macro for impl_247 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_247 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_247"}
// Dependencies: {}
impl < D : Datelike > From < Range < D > > for RangedDate < D > { fn from (range : Range < D >) -> Self { Self (range . start , range . end) } }
};
}
