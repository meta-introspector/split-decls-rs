// Generated macro for impl_450 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_450 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_450"}
// Dependencies: {}
impl From < Time > for Meridiem { fn from (t : Time) -> Meridiem { if t . hour () < 12 { Meridiem :: AM } else { Meridiem :: PM } } }
};
}
