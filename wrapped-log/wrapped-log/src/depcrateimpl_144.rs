// Generated macro for impl_144 (impl)
macro_rules! Depcrateimpl_144 {
() => {
// Module: crate
// Provides: {"impl_144"}
// Dependencies: {}
impl FromStr for Level { type Err = ParseLevelError ; fn from_str (level : & str) -> Result < Level , Self :: Err > { LOG_LEVEL_NAMES . iter () . position (| & name | name . eq_ignore_ascii_case (level)) . into_iter () . filter (| & idx | idx != 0) . map (| idx | Level :: from_usize (idx) . unwrap ()) . next () . ok_or (ParseLevelError (())) } }
};
}
