// Generated macro for has_doctests (function)
macro_rules! Depcrate_clihas_doctests {
() => {
// Module: crate::cli
// Provides: {"has_doctests"}
// Dependencies: {}
# [doc = " Check if any of the packages have doctests"] fn has_doctests (packages : & [Package]) -> bool { for package in packages { for target in & package . targets { if target . kind . iter () . any (| kind | kind == "custom-build") { continue ; } if let Ok (content) = fs :: read_to_string (& target . src_path) { if content . contains ("/// ```") || content . contains ("//! ```") { return true ; } } } } false }
};
}
