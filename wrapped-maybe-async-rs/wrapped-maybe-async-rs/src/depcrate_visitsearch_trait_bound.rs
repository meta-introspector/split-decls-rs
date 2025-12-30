// Generated macro for search_trait_bound (function)
macro_rules! Depcrate_visitsearch_trait_bound {
() => {
// Module: crate::visit
// Provides: {"search_trait_bound"}
// Dependencies: {}
fn search_trait_bound (generic_type_name : & str , bound : & TypeParamBound ,) -> Vec < (String , PathSegment) > { let mut inputs = vec ! [] ; if let TypeParamBound :: Trait (trait_bound) = bound { let segment = & trait_bound . path . segments [trait_bound . path . segments . len () - 1] ; let name = segment . ident . to_string () ; if name . eq ("Future") { if let PathArguments :: AngleBracketed (args) = & segment . arguments { if let GenericArgument :: AssocType (binding) = & args . args [0] { if let Type :: Path (p) = & binding . ty { inputs . push ((generic_type_name . to_owned () , p . path . segments [0] . clone ())) ; } } } } } inputs }
};
}
