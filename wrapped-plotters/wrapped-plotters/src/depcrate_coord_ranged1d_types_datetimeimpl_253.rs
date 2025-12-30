// Generated macro for impl_253 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_253 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_253"}
// Dependencies: {}
impl < T : TimeValue + Datelike + Clone > ValueFormatter < T > for Monthly < T > { fn format (value : & T) -> String { format ! ("{}-{}" , value . year () , value . month ()) } }
};
}
