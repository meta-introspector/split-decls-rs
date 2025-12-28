macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! get_type_path_and_name {
    () => {
        deps!();
        pub fn get_type_path_and_name (ty : & Type) -> GeneratorResult < (& Type , String) > { match ty { Type :: Path (path) => Ok ((ty , path . path . segments . last () . map (| s | s . ident . to_string ()) . unwrap () ,)) , Type :: Group (TypeGroup { elem , .. }) => get_type_path_and_name (elem) , Type :: TraitObject (trait_object) => Ok ((ty , trait_object . bounds . iter () . find_map (| bound | match bound { TypeParamBound :: Trait (t) => { Some (t . path . segments . last () . map (| s | s . ident . to_string ()) . unwrap ()) } _ => None , }) . unwrap () ,)) , _ => Err (Error :: new_spanned (ty , "Invalid type") . into ()) , } }
    };
}

get_type_path_and_name!()