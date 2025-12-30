// Generated macro for get_type_path_and_name (function)
macro_rules! Depcrate_utilsget_type_path_and_name {
() => {
// Module: crate::utils
// Provides: {"get_type_path_and_name"}
// Dependencies: {}
pub fn get_type_path_and_name (ty : & Type) -> GeneratorResult < (& Type , String) > { match ty { Type :: Path (path) => Ok ((ty , path . path . segments . last () . map (| s | s . ident . to_string ()) . unwrap () ,)) , Type :: Group (TypeGroup { elem , .. }) => get_type_path_and_name (elem) , Type :: TraitObject (trait_object) => Ok ((ty , trait_object . bounds . iter () . find_map (| bound | match bound { TypeParamBound :: Trait (t) => { Some (t . path . segments . last () . map (| s | s . ident . to_string ()) . unwrap ()) } _ => None , }) . unwrap () ,)) , _ => Err (Error :: new_spanned (ty , "Invalid type") . into ()) , } }
};
}
