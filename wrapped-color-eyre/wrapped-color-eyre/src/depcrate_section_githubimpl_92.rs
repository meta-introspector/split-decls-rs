// Generated macro for impl_92 (impl)
macro_rules! Depcrate_section_githubimpl_92 {
() => {
// Module: crate::section::github
// Provides: {"impl_92"}
// Dependencies: {}
impl < T > fmt :: Display for Collapsed < T > where T : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (& self . 0) . with_header ("\n<details>\n\n") . with_footer ("\n</details>") . fmt (f) } }
};
}
