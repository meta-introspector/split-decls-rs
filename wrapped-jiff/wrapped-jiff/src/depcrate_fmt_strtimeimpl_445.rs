// Generated macro for impl_445 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_445 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_445"}
// Dependencies: {}
impl From < Time > for BrokenDownTime { fn from (t : Time) -> BrokenDownTime { BrokenDownTime { hour : Some (t . hour_ranged ()) , minute : Some (t . minute_ranged ()) , second : Some (t . second_ranged ()) , subsec : Some (t . subsec_nanosecond_ranged ()) , meridiem : Some (Meridiem :: from (t)) , .. BrokenDownTime :: default () } } }
};
}
