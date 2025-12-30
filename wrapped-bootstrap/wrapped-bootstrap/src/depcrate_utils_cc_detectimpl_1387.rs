// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_utils_cc_detectimpl_1387 {
() => {
// Module: crate::utils::cc_detect
// Provides: {"impl_1387"}
// Dependencies: {}
impl Language { # [doc = " Returns the executable name for a GCC compiler corresponding to this language."] fn gcc (self) -> & 'static str { match self { Language :: C => "gcc" , Language :: CPlusPlus => "g++" , } } # [doc = " Returns the executable name for a clang-based compiler corresponding to this language."] fn clang (self) -> & 'static str { match self { Language :: C => "clang" , Language :: CPlusPlus => "clang++" , } } }
};
}
