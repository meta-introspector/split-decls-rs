// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_tz_zicimpl_1149 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1149"}
// Dependencies: {}
impl FromStr for ZoneStdoffP { type Err = Error ; fn from_str (stdoff : & str) -> Result < ZoneStdoffP , Error > { let span = parse_span (stdoff) ? . fieldwise () ; Ok (ZoneStdoffP { span }) } }
};
}
