// Generated macro for get_snapshot_filename (function)
macro_rules! Depcrate_runtimeget_snapshot_filename {
() => {
// Module: crate::runtime
// Provides: {"get_snapshot_filename"}
// Dependencies: {}
fn get_snapshot_filename (module_path : & str , assertion_file : & str , snapshot_name : & str , cargo_workspace : & Path , is_doctest : bool ,) -> PathBuf { let root = Path :: new (cargo_workspace) ; let base = Path :: new (assertion_file) ; Settings :: with (| settings | { root . join (base . parent () . unwrap ()) . join (settings . snapshot_path ()) . join ({ use std :: fmt :: Write ; let mut f = String :: new () ; if settings . prepend_module_to_snapshot () { if is_doctest { write ! (& mut f , "doctest_{}__" , base . file_name () . unwrap () . to_string_lossy () . replace ('.' , "_")) . unwrap () ; } else { write ! (& mut f , "{}__" , module_path . replace ("::" , "__")) . unwrap () ; } } write ! (& mut f , "{}.snap" , snapshot_name . replace (& ['/' , '\\'] [..] , "__")) . unwrap () ; f }) }) }
};
}
