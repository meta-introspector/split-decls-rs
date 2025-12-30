// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl Default for Ini { # [doc = " Creates an ini instance with an empty general section. This allows [Ini::general_section]"] # [doc = " and [Ini::with_general_section] to be called without panicking."] fn default () -> Self { let mut result = Ini { sections : Default :: default () , } ; result . sections . insert (None , Default :: default ()) ; result } }
};
}
