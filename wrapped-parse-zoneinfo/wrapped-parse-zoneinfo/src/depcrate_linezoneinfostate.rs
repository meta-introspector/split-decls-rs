// Generated macro for ZoneInfoState (enum)
macro_rules! Depcrate_lineZoneInfoState {
() => {
// Module: crate::line
// Provides: {"ZoneInfoState"}
// Dependencies: {}
enum ZoneInfoState < 'a > { Start , Save { offset : TimeSpec , } , Format { offset : TimeSpec , saving : Saving < 'a > , } , Year { offset : TimeSpec , saving : Saving < 'a > , format : & 'a str , } , Month { offset : TimeSpec , saving : Saving < 'a > , format : & 'a str , year : Year , } , Day { offset : TimeSpec , saving : Saving < 'a > , format : & 'a str , year : Year , month : Month , } , Time { offset : TimeSpec , saving : Saving < 'a > , format : & 'a str , year : Year , month : Month , day : DaySpec , } , }
};
}
