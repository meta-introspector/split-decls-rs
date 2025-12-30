// Generated macro for get_unique_attr (function)
macro_rules! Depcrate_attrsget_unique_attr {
() => {
// Module: crate::attrs
// Provides: {"get_unique_attr"}
// Dependencies: {}
pub fn get_unique_attr < 'a , A : AttributeExt > (sess : & 'a Session , attrs : & 'a [A] , name : Symbol) -> Option < & 'a A > { let mut unique_attr : Option < & A > = None ; for attr in get_attr (sess , attrs , name) { if let Some (duplicate) = unique_attr { sess . dcx () . struct_span_err (attr . span () , format ! ("`{name}` is defined multiple times")) . with_span_note (duplicate . span () , "first definition found here") . emit () ; } else { unique_attr = Some (attr) ; } } unique_attr }
};
}
