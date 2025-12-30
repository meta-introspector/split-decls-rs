// Generated macro for impl_31 (impl)
macro_rules! Depcrate_dfa_denseimpl_31 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_31"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl DFA < & [u32] > { # [doc = " Return a new default dense DFA compiler configuration."] # [doc = ""] # [doc = " This is a convenience routine to avoid needing to import the [`Config`]"] # [doc = " type when customizing the construction of a dense DFA."] pub fn config () -> Config { Config :: new () } # [doc = " Create a new dense DFA builder with the default configuration."] # [doc = ""] # [doc = " This is a convenience routine to avoid needing to import the"] # [doc = " [`Builder`] type in common cases."] pub fn builder () -> Builder { Builder :: new () } }
};
}
