// Generated macro for impl_444 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_444 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_444"}
// Dependencies: {}
impl From < ISOWeekDate > for BrokenDownTime { fn from (wd : ISOWeekDate) -> BrokenDownTime { BrokenDownTime { iso_week_year : Some (wd . year_ranged ()) , iso_week : Some (wd . week_ranged ()) , weekday : Some (wd . weekday ()) , .. BrokenDownTime :: default () } } }
};
}
