// Generated macro for extract_type_from_option (function)
macro_rules! Depcrate_setterextract_type_from_option {
() => {
// Module: crate::setter
// Provides: {"extract_type_from_option"}
// Dependencies: {}
fn extract_type_from_option (ty : & syn :: Type) -> Option < & syn :: Type > { use syn :: punctuated :: Pair ; use syn :: token :: PathSep ; use syn :: { GenericArgument , Path , PathArguments , PathSegment } ; fn extract_type_path (ty : & syn :: Type) -> Option < & Path > { match * ty { syn :: Type :: Path (ref typepath) if typepath . qself . is_none () => Some (& typepath . path) , _ => None , } } fn extract_option_segment (path : & Path) -> Option < Pair < & PathSegment , & PathSep > > { let idents_of_path = path . segments . iter () . fold (String :: new () , | mut acc , v | { acc . push_str (& v . ident . to_string ()) ; acc . push ('|') ; acc }) ; vec ! ["Option|" , "std|option|Option|" , "core|option|Option|"] . into_iter () . find (| s | idents_of_path == * s) . and_then (| _ | path . segments . last () . map (Pair :: End)) } extract_type_path (ty) . and_then (extract_option_segment) . and_then (| pair_path_segment | { let type_params = & pair_path_segment . into_value () . arguments ; match * type_params { PathArguments :: AngleBracketed (ref params) => params . args . first () , _ => None , } }) . and_then (| generic_arg | match * generic_arg { GenericArgument :: Type (ref ty) => Some (ty) , _ => None , }) }
};
}
