// Generated macro for impl_442 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_442 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_442"}
// Dependencies: {}
impl From < DateTime > for BrokenDownTime { fn from (dt : DateTime) -> BrokenDownTime { let (d , t) = (dt . date () , dt . time ()) ; BrokenDownTime { year : Some (d . year_ranged ()) , month : Some (d . month_ranged ()) , day : Some (d . day_ranged ()) , hour : Some (t . hour_ranged ()) , minute : Some (t . minute_ranged ()) , second : Some (t . second_ranged ()) , subsec : Some (t . subsec_nanosecond_ranged ()) , meridiem : Some (Meridiem :: from (t)) , .. BrokenDownTime :: default () } } }
};
}
