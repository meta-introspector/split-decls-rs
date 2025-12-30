// Generated macro for extract_args_attrs (function)
macro_rules! Depcrate_parserextract_args_attrs {
() => {
// Module: crate::parser
// Provides: {"extract_args_attrs"}
// Dependencies: {}
# [doc = " Extracts function arguments attributes"] fn extract_args_attrs (sig : & mut syn :: Signature) -> Result < Vec < FnArgAttrs > , Diagnostic > { let mut args_attrs = vec ! [] ; for input in sig . inputs . iter_mut () { if let syn :: FnArg :: Typed (pat_type) = input { let attrs = BindgenAttrs :: find (& mut pat_type . attrs) ? ; let arg_attrs = FnArgAttrs { js_name : attrs . js_name () . map_or (Ok (None) , | (js_name_override , span) | { if is_js_keyword (js_name_override) || ! is_valid_ident (js_name_override) { return Err (Diagnostic :: span_error (span , "invalid JS identifier")) ; } Ok (Some (js_name_override . to_string ())) }) ? , js_type : attrs . unchecked_param_type () . map_or :: < Result < _ , Diagnostic > , _ > (Ok (None) , | (ty , span) | { check_invalid_type (ty , span) ? ; Ok (Some (ty . to_string ())) }) ? , desc : attrs . param_description () . map_or :: < Result < _ , Diagnostic > , _ > (Ok (None) , | (description , span) | { check_js_comment_close (description , span) ? ; Ok (Some (description . to_string ())) }) ? , } ; attrs . enforce_used () ? ; args_attrs . push (arg_attrs) ; } } Ok (args_attrs) }
};
}
