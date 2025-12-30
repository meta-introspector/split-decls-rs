// Generated macro for impl_443 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_443 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_443"}
// Dependencies: {}
impl From < Date > for BrokenDownTime { fn from (d : Date) -> BrokenDownTime { BrokenDownTime { year : Some (d . year_ranged ()) , month : Some (d . month_ranged ()) , day : Some (d . day_ranged ()) , .. BrokenDownTime :: default () } } }
};
}
