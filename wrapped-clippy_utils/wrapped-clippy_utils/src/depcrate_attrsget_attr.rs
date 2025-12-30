// Generated macro for get_attr (function)
macro_rules! Depcrate_attrsget_attr {
() => {
// Module: crate::attrs
// Provides: {"get_attr"}
// Dependencies: {}
pub fn get_attr < 'a , A : AttributeExt + 'a > (sess : & 'a Session , attrs : & 'a [A] , name : Symbol ,) -> impl Iterator < Item = & 'a A > { attrs . iter () . filter (move | attr | { let Some (attr_segments) = attr . ident_path () else { return false ; } ; if attr_segments . len () == 2 && attr_segments [0] . name == sym :: clippy { BUILTIN_ATTRIBUTES . iter () . find_map (| (builtin_name , deprecation_status) | { if attr_segments [1] . name == * builtin_name { Some (deprecation_status) } else { None } }) . map_or_else (| | { sess . dcx () . span_err (attr_segments [1] . span , "usage of unknown attribute") ; false } , | deprecation_status | { let mut diag = sess . dcx () . struct_span_err (attr_segments [1] . span , "usage of deprecated attribute") ; match * deprecation_status { DeprecationStatus :: Deprecated => { diag . emit () ; false } , DeprecationStatus :: Replaced (new_name) => { diag . span_suggestion (attr_segments [1] . span , "consider using" , new_name , Applicability :: MachineApplicable ,) ; diag . emit () ; false } , DeprecationStatus :: None => { diag . cancel () ; attr_segments [1] . name == name } , } } ,) } else { false } }) }
};
}
