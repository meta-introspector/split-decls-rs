// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl MatchOptions { # [doc = " Constructs a new `MatchOptions` with default field values. This is used"] # [doc = " when calling functions that do not take an explicit `MatchOptions`"] # [doc = " parameter."] # [doc = ""] # [doc = " This function always returns this value:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " MatchOptions {"] # [doc = "     case_sensitive: true,"] # [doc = "     require_literal_separator: false,"] # [doc = "     require_literal_leading_dot: false"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Note"] # [doc = " The behavior of this method doesn't match `default()`'s. This returns"] # [doc = " `case_sensitive` as `true` while `default()` does it as `false`."] pub fn new () -> Self { Self { case_sensitive : true , require_literal_separator : false , require_literal_leading_dot : false , } } }
};
}
