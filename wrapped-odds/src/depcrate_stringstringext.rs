// Generated macro for StringExt (trait)
macro_rules! Depcrate_stringStringExt {
() => {
// Module: crate::string
// Provides: {"StringExt"}
// Dependencies: {}
# [cfg (feature = "std-string")] # [doc = " Extra methods for `String`"] # [doc = ""] # [doc = " Requires `feature=\"std-string\"`"] pub trait StringExt { # [doc = " **Panics** if `index` is out of bounds."] # [deprecated (note = "Use String::insert_str")] fn insert_str (& mut self , index : usize , s : & str) ; }
};
}
