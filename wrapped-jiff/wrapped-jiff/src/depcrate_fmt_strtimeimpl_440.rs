// Generated macro for impl_440 (impl)
macro_rules! Depcrate_fmt_strtimeimpl_440 {
() => {
// Module: crate::fmt::strtime
// Provides: {"impl_440"}
// Dependencies: {}
impl < 'a > From < & 'a Zoned > for BrokenDownTime { fn from (zdt : & 'a Zoned) -> BrokenDownTime { # [cfg (feature = "alloc")] let iana = { use alloc :: string :: ToString ; zdt . time_zone () . iana_name () . map (| s | s . to_string ()) } ; BrokenDownTime { offset : Some (zdt . offset ()) , timestamp : Some (zdt . timestamp ()) , tz : Some (zdt . time_zone () . clone ()) , # [cfg (feature = "alloc")] iana , .. BrokenDownTime :: from (zdt . datetime ()) } } }
};
}
