// Generated macro for impl_20 (impl)
macro_rules! Depcrate_lineimpl_20 {
() => {
// Module: crate::line
// Provides: {"impl_20"}
// Dependencies: {}
impl TimeSpec { # [doc = " Returns the number of seconds past midnight that this time spec"] # [doc = " represents."] pub fn as_seconds (self) -> i64 { match self { TimeSpec :: Hours (h) => h as i64 * 60 * 60 , TimeSpec :: HoursMinutes (h , m) => h as i64 * 60 * 60 + m as i64 * 60 , TimeSpec :: HoursMinutesSeconds (h , m , s) => h as i64 * 60 * 60 + m as i64 * 60 + s as i64 , TimeSpec :: Zero => 0 , } } pub fn with_type (self , timetype : TimeType) -> TimeSpecAndType { TimeSpecAndType (self , timetype) } }
};
}
