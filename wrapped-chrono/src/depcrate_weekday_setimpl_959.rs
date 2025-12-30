// Generated macro for impl_959 (impl)
macro_rules! Depcrate_weekday_setimpl_959 {
() => {
// Module: crate::weekday_set
// Provides: {"impl_959"}
// Dependencies: {}
impl FromIterator < Weekday > for WeekdaySet { fn from_iter < T : IntoIterator < Item = Weekday > > (iter : T) -> Self { iter . into_iter () . map (Self :: single) . fold (Self :: EMPTY , Self :: union) } }
};
}
