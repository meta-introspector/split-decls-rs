// Generated macro for FieldOrMethodPattern (struct)
macro_rules! Depcrate_matches_patternFieldOrMethodPattern {
() => {
// Module: crate::matches_pattern
// Provides: {"FieldOrMethodPattern"}
// Dependencies: {}
# [doc = " Either field or method call matcher. E.g.:"] # [doc = " * `field: starts_with(\"something\")` or `field: _`"] # [doc = " * `property(arg1, arg2): starts_with(\"something\")"] struct FieldOrMethodPattern { ref_token : Option < Token ! [ref] > , field_or_method : FieldOrMethod , # [doc = " When `None`, it represents `_` which matches anything, meaning we should"] # [doc = " ignore it."] matcher : Option < Expr > , }
};
}
