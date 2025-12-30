// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for StyledCalendar { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Default => write ! (f , "Default") , Self :: Surrounding => write ! (f , "Show Surrounding") , Self :: WeekdaysHeader => write ! (f , "Show Weekdays Header") , Self :: SurroundingAndWeekdaysHeader => write ! (f , "Show Surrounding and Weekdays Header") , Self :: MonthHeader => write ! (f , "Show Month Header") , Self :: MonthAndWeekdaysHeader => write ! (f , "Show Month Header and Weekdays Header") , } } }
};
}
