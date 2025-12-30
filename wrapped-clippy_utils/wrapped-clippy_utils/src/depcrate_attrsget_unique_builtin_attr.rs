// Generated macro for get_unique_builtin_attr (function)
macro_rules! Depcrate_attrsget_unique_builtin_attr {
() => {
// Module: crate::attrs
// Provides: {"get_unique_builtin_attr"}
// Dependencies: {}
# [doc = " If `attrs` contain exactly one instance of a built-in Clippy attribute called `name`,"] # [doc = " returns that attribute, and `None` otherwise"] pub fn get_unique_builtin_attr < 'a , A : AttributeExt > (sess : & 'a Session , attrs : & 'a [A] , name : Symbol) -> Option < & 'a A > { let mut unique_attr : Option < & A > = None ; for attr in get_builtin_attr (sess , attrs , name) { if let Some (duplicate) = unique_attr { sess . dcx () . struct_span_err (attr . span () , format ! ("`{name}` is defined multiple times")) . with_span_note (duplicate . span () , "first definition found here") . emit () ; } else { unique_attr = Some (attr) ; } } unique_attr }
};
}
