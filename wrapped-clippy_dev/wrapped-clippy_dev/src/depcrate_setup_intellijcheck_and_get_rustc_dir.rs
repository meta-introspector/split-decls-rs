// Generated macro for check_and_get_rustc_dir (function)
macro_rules! Depcrate_setup_intellijcheck_and_get_rustc_dir {
() => {
// Module: crate::setup::intellij
// Provides: {"check_and_get_rustc_dir"}
// Dependencies: {}
fn check_and_get_rustc_dir (rustc_path : & str) -> Result < PathBuf , () > { let mut path = PathBuf :: from (rustc_path) ; if path . is_relative () { match path . canonicalize () { Ok (absolute_path) => { println ! ("info: the rustc path was resolved to: `{}`" , absolute_path . display ()) ; path = absolute_path ; } , Err (err) => { eprintln ! ("error: unable to get the absolute path of rustc ({err})") ; return Err (()) ; } , } } let path = path . join ("compiler") ; println ! ("info: looking for compiler sources at: {}" , path . display ()) ; if ! path . exists () { eprintln ! ("error: the given path does not exist") ; return Err (()) ; } if ! path . is_dir () { eprintln ! ("error: the given path is not a directory") ; return Err (()) ; } Ok (path) }
};
}
