// Generated macro for get_recursive_paths (function)
macro_rules! Depcrate_tarballerget_recursive_paths {
() => {
// Module: crate::tarballer
// Provides: {"get_recursive_paths"}
// Dependencies: {}
# [doc = " Returns all `(directories, files)` under the source path."] fn get_recursive_paths < P , Q > (root : P , name : Q) -> Result < (Vec < String > , Vec < String >) > where P : AsRef < Path > , Q : AsRef < Path > , { let root = root . as_ref () ; let name = name . as_ref () ; if ! name . is_relative () && ! name . starts_with (root) { bail ! ("input '{}' is not in work dir '{}'" , name . display () , root . display ()) ; } let mut dirs = vec ! [] ; let mut files = vec ! [] ; for entry in WalkDir :: new (root . join (name)) { let entry = entry ? ; let path = entry . path () . strip_prefix (root) ? ; let path = path_to_str (path) ? ; if entry . file_type () . is_dir () { dirs . push (path . to_owned ()) ; } else { files . push (path . to_owned ()) ; } } Ok ((dirs , files)) }
};
}
