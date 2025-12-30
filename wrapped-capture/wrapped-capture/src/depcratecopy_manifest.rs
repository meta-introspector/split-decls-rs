// Generated macro for copy_manifest (function)
macro_rules! Depcratecopy_manifest {
() => {
// Module: crate
// Provides: {"copy_manifest"}
// Dependencies: {}
fn copy_manifest < W : std :: io :: Write > (manifest_path : & Path , ar : & mut tar :: Builder < W > , name : & Path , source_root : & Path ,) { let relative_path = manifest_path . parent () . unwrap () . strip_prefix (source_root) . expect ("workspace member should be under workspace root") ; let relative_path = name . join (relative_path) ; let contents = fs :: read_to_string (& manifest_path) . unwrap () ; let mut manifest : toml :: Value = toml :: from_str (& contents) . unwrap () ; let remove = | obj : & mut toml :: Value , name | { let table = obj . as_table_mut () . unwrap () ; if table . contains_key (name) { table . remove (name) ; } } ; remove (& mut manifest , "lib") ; remove (& mut manifest , "bin") ; remove (& mut manifest , "example") ; remove (& mut manifest , "test") ; remove (& mut manifest , "bench") ; remove (& mut manifest , "profile") ; if let Some (package) = manifest . get_mut ("package") { remove (package , "default-run") ; } let contents = toml :: to_string (& manifest) . unwrap () ; add_ar_file (ar , & relative_path . join ("Cargo.toml") , & contents) ; add_ar_file (ar , & relative_path . join ("src") . join ("lib.rs") , "") ; }
};
}
