// Generated macro for root_macro_call (function)
macro_rules! Depcrate_macrosroot_macro_call {
() => {
// Module: crate::macros
// Provides: {"root_macro_call"}
// Dependencies: {}
# [doc = " If the macro backtrace of `span` has a macro call at the root expansion"] # [doc = " (i.e. not a nested macro call), returns `Some` with the `MacroCall`"] # [doc = ""] # [doc = " If you only want to check whether the root macro has a specific name,"] # [doc = " consider using [`matching_root_macro_call`] instead."] pub fn root_macro_call (span : Span) -> Option < MacroCall > { macro_backtrace (span) . last () }
};
}
