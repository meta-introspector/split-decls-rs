// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl LintInfo { # [doc = " Returns the lint name in lowercase without the `clippy::` prefix"] # [must_use] # [expect (clippy :: missing_panics_doc)] pub fn name_lower (& self) -> String { self . lint . name . strip_prefix ("clippy::") . unwrap () . to_ascii_lowercase () } }
};
}
