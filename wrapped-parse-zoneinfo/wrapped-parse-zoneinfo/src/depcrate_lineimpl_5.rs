// Generated macro for impl_5 (impl)
macro_rules! Depcrate_lineimpl_5 {
() => {
// Module: crate::line
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: FailedYearParse (s) => write ! (f , "failed to parse as a year value: \"{}\"" , s) , Error :: FailedMonthParse (s) => write ! (f , "failed to parse as a month value: \"{}\"" , s) , Error :: FailedWeekdayParse (s) => { write ! (f , "failed to parse as a weekday value: \"{}\"" , s) } Error :: InvalidLineType (s) => write ! (f , "line with invalid format: \"{}\"" , s) , Error :: TypeColumnContainedNonHyphen (s) => { write ! (f , "'type' column is not a hyphen but has the value: \"{}\"" , s) } Error :: CouldNotParseSaving (s) => write ! (f , "failed to parse RULES column: \"{}\"" , s) , Error :: InvalidDaySpec (s) => write ! (f , "invalid day specification ('ON'): \"{}\"" , s) , Error :: InvalidTimeSpecAndType (s) => write ! (f , "invalid time: \"{}\"" , s) , Error :: NonWallClockInTimeSpec (s) => { write ! (f , "time value not given as wall time: \"{}\"" , s) } Error :: NotParsedAsRuleLine => write ! (f , "failed to parse line as a rule") , Error :: NotParsedAsZoneLine => write ! (f , "failed to parse line as a zone") , Error :: NotParsedAsLinkLine => write ! (f , "failed to parse line as a link") , } } }
};
}
