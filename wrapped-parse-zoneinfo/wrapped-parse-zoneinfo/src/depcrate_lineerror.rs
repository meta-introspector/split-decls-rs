// Generated macro for Error (enum)
macro_rules! Depcrate_lineError {
() => {
// Module: crate::line
// Provides: {"Error"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone)] pub enum Error { FailedYearParse (String) , FailedMonthParse (String) , FailedWeekdayParse (String) , InvalidLineType (String) , TypeColumnContainedNonHyphen (String) , CouldNotParseSaving (String) , InvalidDaySpec (String) , InvalidTimeSpecAndType (String) , NonWallClockInTimeSpec (String) , NotParsedAsRuleLine , NotParsedAsZoneLine , NotParsedAsLinkLine , }
};
}
