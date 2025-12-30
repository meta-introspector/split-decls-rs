// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
impl FromStr for LevelFilter { type Err = ParseLevelError ; fn from_str (level : & str) -> Result < LevelFilter , Self :: Err > { LOG_LEVEL_NAMES . iter () . position (| & name | name . eq_ignore_ascii_case (level)) . map (| p | LevelFilter :: from_usize (p) . unwrap ()) . ok_or (ParseLevelError (())) } }
};
}
