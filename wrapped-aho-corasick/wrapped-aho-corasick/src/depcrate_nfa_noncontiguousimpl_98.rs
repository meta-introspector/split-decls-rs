// Generated macro for impl_98 (impl)
macro_rules! Depcrate_nfa_noncontiguousimpl_98 {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"impl_98"}
// Dependencies: {}
impl NFA { # [doc = " Create a new Aho-Corasick noncontiguous NFA using the default"] # [doc = " configuration."] # [doc = ""] # [doc = " Use a [`Builder`] if you want to change the configuration."] pub fn new < I , P > (patterns : I) -> Result < NFA , BuildError > where I : IntoIterator < Item = P > , P : AsRef < [u8] > , { NFA :: builder () . build (patterns) } # [doc = " A convenience method for returning a new Aho-Corasick noncontiguous NFA"] # [doc = " builder."] # [doc = ""] # [doc = " This usually permits one to just import the `NFA` type."] pub fn builder () -> Builder { Builder :: new () } }
};
}
