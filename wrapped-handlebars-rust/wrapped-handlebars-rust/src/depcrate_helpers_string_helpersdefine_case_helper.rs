// Generated macro for define_case_helper (macro)
macro_rules! Depcrate_helpers_string_helpersdefine_case_helper {
() => {
// Module: crate::helpers::string_helpers
// Provides: {"define_case_helper"}
// Dependencies: {}
macro_rules ! define_case_helper { ($ helper_fn_name : ident , $ heck_fn_name : ident) => { pub (crate) fn $ helper_fn_name (h : & crate :: render :: Helper <'_ >, _ : & crate :: Handlebars <'_ >, _ : & crate :: context :: Context , _rc : & mut crate :: render :: RenderContext <'_ , '_ >, out : & mut dyn crate :: output :: Output ,) -> crate :: helpers :: HelperResult { let param = h . param (0) . and_then (| v | v . value () . as_str ()) . ok_or_else (|| { crate :: error :: RenderErrorReason :: ParamTypeMismatchForName (stringify ! ($ helper_fn_name) , "0" . to_owned () , "string" . to_owned () ,) }) ?; out . write (param .$ heck_fn_name () . as_ref ()) ?; Ok (()) } } ; }
};
}
