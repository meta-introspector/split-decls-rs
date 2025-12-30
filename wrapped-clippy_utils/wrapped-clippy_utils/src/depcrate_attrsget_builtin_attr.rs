// Generated macro for get_builtin_attr (function)
macro_rules! Depcrate_attrsget_builtin_attr {
() => {
// Module: crate::attrs
// Provides: {"get_builtin_attr"}
// Dependencies: {}
# [doc = " Given `attrs`, extract all the instances of a built-in Clippy attribute called `name`"] pub fn get_builtin_attr < 'a , A : AttributeExt + 'a > (sess : & 'a Session , attrs : & 'a [A] , name : Symbol ,) -> impl Iterator < Item = & 'a A > { attrs . iter () . filter (move | attr | { if let Some ([clippy , segment2]) = attr . ident_path () . as_deref () && clippy . name == sym :: clippy { let new_name = match segment2 . name { sym :: cyclomatic_complexity => Some ("cognitive_complexity") , sym :: author | sym :: version | sym :: cognitive_complexity | sym :: dump | sym :: msrv | sym :: has_significant_drop | sym :: format_args => None , _ => { sess . dcx () . span_err (segment2 . span , "usage of unknown attribute") ; return false ; } , } ; match new_name { Some (new_name) => { sess . dcx () . struct_span_err (segment2 . span , "usage of deprecated attribute") . with_span_suggestion (segment2 . span , "consider using" , new_name , Applicability :: MachineApplicable ,) . emit () ; false } , None => segment2 . name == name , } } else { false } }) }
};
}
