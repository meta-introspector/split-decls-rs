// Generated macro for impl_866 (impl)
macro_rules! Depcrate_http_altair_sourceimpl_866 {
() => {
// Module: crate::http::altair_source
// Provides: {"impl_866"}
// Dependencies: {}
impl handlebars :: HelperDef for ToJsonHelper { # [allow (unused_assignments)] fn call_inner < 'reg : 'rc , 'rc > (& self , h : & handlebars :: Helper < 'rc > , r : & 'reg handlebars :: Handlebars < 'reg > , _ : & 'rc handlebars :: Context , _ : & mut handlebars :: RenderContext < 'reg , 'rc > ,) -> std :: result :: Result < handlebars :: ScopedJson < 'rc > , handlebars :: RenderError > { let mut param_idx = 0 ; let obj = h . param (param_idx) . and_then (| x | { if r . strict_mode () && x . is_value_missing () { None } else { Some (x . value ()) } }) . ok_or_else (| | { handlebars :: RenderErrorReason :: ParamNotFoundForName ("toJson" , "obj" . to_string ()) }) . and_then (| x | { x . as_object () . ok_or_else (| | { handlebars :: RenderErrorReason :: ParamTypeMismatchForName ("toJson" , "obj" . to_string () , "object" . to_string () ,) }) }) ? ; param_idx += 1 ; let result = if obj . is_empty () { "{}" . to_owned () } else { serde_json :: to_string (& obj) . expect ("Failed to serialize json") } ; Ok (handlebars :: ScopedJson :: Derived (handlebars :: JsonValue :: from (result) ,)) } }
};
}
