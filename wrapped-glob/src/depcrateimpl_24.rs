// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl Error for GlobError { # [allow (deprecated)] fn description (& self) -> & str { self . error . description () } # [allow (unknown_lints , bare_trait_objects)] fn cause (& self) -> Option < & Error > { Some (& self . error) } }
};
}
