// Generated macro for impl_201 (impl)
macro_rules! Depcrate_errorimpl_201 {
() => {
// Module: crate::error
// Provides: {"impl_201"}
// Dependencies: {}
impl Position { pub (crate) fn from_src_end (src : & str) -> Position { let line = 1 + src . chars () . filter (| & c | c == '\n') . count () ; let col = 1 + src . chars () . rev () . take_while (| & c | c != '\n') . count () ; Self { line , col } } }
};
}
