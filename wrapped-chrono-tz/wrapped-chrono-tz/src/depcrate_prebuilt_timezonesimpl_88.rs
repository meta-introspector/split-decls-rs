// Generated macro for impl_88 (impl)
macro_rules! Depcrate_prebuilt_timezonesimpl_88 {
() => {
// Module: crate::prebuilt::timezones
// Provides: {"impl_88"}
// Dependencies: {}
impl FromStr for Tz { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { TIMEZONES . get (s) . cloned () . ok_or (ParseError (())) } }
};
}
