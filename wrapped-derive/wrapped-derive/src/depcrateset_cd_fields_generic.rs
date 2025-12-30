// Generated macro for set_cd_fields_generic (function)
macro_rules! Depcrateset_cd_fields_generic {
() => {
// Module: crate
// Provides: {"set_cd_fields_generic"}
// Dependencies: {}
# [cfg (feature = "conditional_deserialization")] fn set_cd_fields_generic (mut item_struct : ItemStruct , value : proc_macro2 :: TokenStream ,) -> ItemStruct { use syn :: { AngleBracketedGenericArguments , PathArguments , parse :: { Parse , Parser } , } ; item_struct . fields . iter_mut () . for_each (| field | { if function_cd (field) . unwrap () { if let Type :: Path (path) = & mut field . ty { if let Some (segment) = path . path . segments . last_mut () { if let PathArguments :: AngleBracketed (argument) = & mut segment . arguments { argument . args . push (parse_quote ! { # value }) ; } else { let parser = AngleBracketedGenericArguments :: parse ; let angle_bracketed = parser . parse (TokenStream :: from (quote ! (<# value >))) ; let angle_bracketed = angle_bracketed . unwrap () ; segment . arguments = PathArguments :: AngleBracketed (angle_bracketed) ; } } } else { panic ! ("Only simple types are supported for conditional deserialization.") ; } } }) ; item_struct }
};
}
