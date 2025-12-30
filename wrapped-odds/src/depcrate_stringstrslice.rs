// Generated macro for StrSlice (trait)
macro_rules! Depcrate_stringStrSlice {
() => {
// Module: crate::string
// Provides: {"StrSlice"}
// Dependencies: {}
# [doc = " Extension trait for `str` for string slicing without panicking"] # [deprecated (note = "Use str::get with a range instead")] pub trait StrSlice { # [doc = " Return a slice of the string, if it is in bounds /and on character boundaries/,"] # [doc = " otherwise return `None`"] # [deprecated (note = "Use str::get with a range instead")] fn get_slice < R > (& self , r : R) -> Option < & str > where R : IndexRange ; }
};
}
