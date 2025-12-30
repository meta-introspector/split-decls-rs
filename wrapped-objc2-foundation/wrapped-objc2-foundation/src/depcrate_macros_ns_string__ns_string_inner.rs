// Generated macro for __ns_string_inner (macro)
macro_rules! Depcrate_macros_ns_string__ns_string_inner {
() => {
// Module: crate::macros::ns_string
// Provides: {"__ns_string_inner"}
// Dependencies: {}
# [doc (hidden)] # [cfg (not (all (target_vendor = "apple" , feature = "unstable-static-nsstring")))] # [macro_export] macro_rules ! __ns_string_inner { ($ inp : ident) => { { static CACHED_NSSTRING : $ crate :: __ns_macro_helpers :: CachedRetained <$ crate :: NSString > = $ crate :: __ns_macro_helpers :: CachedRetained :: new () ; CACHED_NSSTRING . get (|| $ crate :: NSString :: from_str ($ inp)) } } ; }
};
}
