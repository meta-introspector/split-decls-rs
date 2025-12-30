// Generated macro for impl_259 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_259 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_259"}
// Dependencies: {}
impl < T : TimeValue + Datelike + Clone > ValueFormatter < T > for Yearly < T > { fn format (value : & T) -> String { format ! ("{}-{}" , value . year () , value . month ()) } }
};
}
