mkuse!{pub use super :: common :: Args ;}
mkuse!{use crate :: sys :: pal :: os :: get_application_parameters ;}
mkuse!{use crate :: sys :: pal :: os :: params :: ArgumentList ;}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    pub fn args () -> Args { let Some (params) = get_application_parameters () else { return Args :: new (vec ! []) ; } ; for param in params { if let Ok (args) = ArgumentList :: try_from (& param) { let mut parsed_args = vec ! [] ; for arg in args { parsed_args . push (arg . into ()) ; } return Args :: new (parsed_args) ; } } Args :: new (vec ! []) }
}