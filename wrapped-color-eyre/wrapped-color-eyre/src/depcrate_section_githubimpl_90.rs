// Generated macro for impl_90 (impl)
macro_rules! Depcrate_section_githubimpl_90 {
() => {
// Module: crate::section::github
// Provides: {"impl_90"}
// Dependencies: {}
impl < T > fmt :: Display for ConsoleSection < T > where T : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (& self . 0) . with_header ("```\n") . with_footer ("\n```") . fmt (f) } }
};
}
