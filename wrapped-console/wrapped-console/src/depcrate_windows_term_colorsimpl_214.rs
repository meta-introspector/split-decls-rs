// Generated macro for impl_214 (impl)
macro_rules! Depcrate_windows_term_colorsimpl_214 {
() => {
// Module: crate::windows_term::colors
// Provides: {"impl_214"}
// Dependencies: {}
impl HandleKind { fn handle (& self) -> HANDLE { match * self { HandleKind :: Stdout => io :: stdout () . as_raw_handle () as HANDLE , HandleKind :: Stderr => io :: stderr () . as_raw_handle () as HANDLE , } } }
};
}
