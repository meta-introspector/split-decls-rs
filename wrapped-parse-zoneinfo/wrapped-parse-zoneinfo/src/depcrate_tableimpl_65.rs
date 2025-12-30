// Generated macro for impl_65 (impl)
macro_rules! Depcrate_tableimpl_65 {
() => {
// Module: crate::table
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'line > From < line :: ZoneInfo < 'line > > for ZoneInfo { fn from (info : line :: ZoneInfo) -> ZoneInfo { ZoneInfo { offset : info . utc_offset . as_seconds () , saving : match info . saving { line :: Saving :: NoSaving => Saving :: NoSaving , line :: Saving :: Multiple (s) => Saving :: Multiple (s . to_owned ()) , line :: Saving :: OneOff (t) => Saving :: OneOff (t . as_seconds ()) , } , format : Format :: new (info . format) , end_time : info . time , } } }
};
}
