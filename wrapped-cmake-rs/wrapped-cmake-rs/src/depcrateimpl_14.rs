// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Version { fn new (major : u32 , minor : u32) -> Self { Self { major , minor } } fn parse (s : & str) -> Option < Self > { let version = s . lines () . next () ? . strip_prefix ("cmake version ") ? ; let mut digits = version . splitn (3 , '.') ; let major = digits . next () ? . parse :: < u32 > () . ok () ? ; let minor = digits . next () ? . parse :: < u32 > () . ok () ? ; Some (Version :: new (major , minor)) } fn from_command (executable : & OsStr) -> Option < Self > { let output = Command :: new (executable) . arg ("--version") . output () . ok () ? ; if ! output . status . success () { return None ; } let stdout = core :: str :: from_utf8 (& output . stdout) . ok () ? ; Self :: parse (stdout) } }
};
}
