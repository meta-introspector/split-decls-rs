// Generated macro for impl_38 (impl)
macro_rules! Depcrate_parseimpl_38 {
() => {
// Module: crate::parse
// Provides: {"impl_38"}
// Dependencies: {}
impl FromStr for Time { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parse_header (s) . ok_or_else (| | Error :: InvalidDateString { input : s . into () }) } }
};
}
