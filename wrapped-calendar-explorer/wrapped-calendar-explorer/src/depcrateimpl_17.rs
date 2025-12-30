// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl StyledCalendar { const fn next (self) -> Self { match self { Self :: Default => Self :: Surrounding , Self :: Surrounding => Self :: WeekdaysHeader , Self :: WeekdaysHeader => Self :: SurroundingAndWeekdaysHeader , Self :: SurroundingAndWeekdaysHeader => Self :: MonthHeader , Self :: MonthHeader => Self :: MonthAndWeekdaysHeader , Self :: MonthAndWeekdaysHeader => Self :: Default , } } }
};
}
