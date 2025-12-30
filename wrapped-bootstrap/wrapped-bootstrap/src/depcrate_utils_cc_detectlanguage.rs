// Generated macro for Language (enum)
macro_rules! Depcrate_utils_cc_detectLanguage {
() => {
// Module: crate::utils::cc_detect
// Provides: {"Language"}
// Dependencies: {}
# [doc = " Representing the target programming language for a native compiler."] # [doc = ""] # [doc = " This enum is used to indicate whether a particular compiler is intended for C or C++."] # [doc = " It also provides helper methods for obtaining the standard executable names for GCC and"] # [doc = " clang-based compilers."] # [derive (PartialEq)] pub (crate) enum Language { # [doc = " The compiler is targeting C."] C , # [doc = " The compiler is targeting C++."] CPlusPlus , }
};
}
