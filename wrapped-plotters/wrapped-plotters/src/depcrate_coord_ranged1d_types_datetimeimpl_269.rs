// Generated macro for impl_269 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_269 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_269"}
// Dependencies: {}
impl From < Range < NaiveDateTime > > for RangedDateTime < NaiveDateTime > { fn from (range : Range < NaiveDateTime >) -> Self { Self (range . start , range . end) } }
};
}
