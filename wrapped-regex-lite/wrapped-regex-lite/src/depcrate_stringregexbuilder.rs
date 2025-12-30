// Generated macro for RegexBuilder (struct)
macro_rules! Depcrate_stringRegexBuilder {
() => {
// Module: crate::string
// Provides: {"RegexBuilder"}
// Dependencies: {}
# [doc = " A configurable builder for a [`Regex`]."] # [doc = ""] # [doc = " This builder can be used to programmatically set flags such as `i` (case"] # [doc = " insensitive) and `x` (for verbose mode). This builder can also be used to"] # [doc = " configure things like a size limit on the compiled regular expression."] # [derive (Debug)] pub struct RegexBuilder { pattern : String , hir_config : hir :: Config , nfa_config : nfa :: Config , }
};
}
