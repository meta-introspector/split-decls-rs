// Generated macro for is_from_proc_macro (function)
macro_rules! Depcrate_check_proc_macrois_from_proc_macro {
() => {
// Module: crate::check_proc_macro
// Provides: {"is_from_proc_macro"}
// Dependencies: {}
# [doc = " Checks if the item likely came from a proc-macro."] # [doc = ""] # [doc = " This should be called after `in_external_macro` and the initial pattern matching of the ast as"] # [doc = " it is significantly slower than both of those."] pub fn is_from_proc_macro < 'cx , T : WithSearchPat < 'cx > > (cx : & T :: Context , item : & T) -> bool { let (start_pat , end_pat) = item . search_pat (cx) ; ! span_matches_pat (cx . sess () , item . span () , start_pat , end_pat) }
};
}
