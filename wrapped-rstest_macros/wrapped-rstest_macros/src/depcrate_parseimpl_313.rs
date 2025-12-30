// Generated macro for impl_313 (impl)
macro_rules! Depcrate_parseimpl_313 {
() => {
// Module: crate::parse
// Provides: {"impl_313"}
// Dependencies: {}
impl VisitMut for ExcludedTraceAttributesFunctionExtractor { fn visit_fn_arg_mut (& mut self , node : & mut FnArg) { let pat = match node . maybe_pat () . cloned () { Some (pat) => pat , None => return , } ; for r in extract_argument_attrs (node , | a | attr_is (a , "notrace") , | _a | Ok (())) { match r { Ok (_) => self . update_excluded (pat . clone ()) , Err (err) => self . update_error (err . into ()) , } } syn :: visit_mut :: visit_fn_arg_mut (self , node) ; } }
};
}
