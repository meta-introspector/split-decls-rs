// Generated macro for scan_decls (function)
macro_rules! Depcratescan_decls {
() => {
// Module: crate
// Provides: {"scan_decls"}
// Dependencies: {}
fn scan_decls (dir : & Path , crate_name : & str) -> Result < () , Box < dyn std :: error :: Error > > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { let file_name = path . file_stem () . unwrap () . to_string_lossy () ; let relative_path = path . strip_prefix ("../output2") . unwrap_or (& path) ; println ! ("{} = \"{}\"" , file_name , relative_path . display ()) ; } else if path . is_dir () { scan_decls (& path , crate_name) ? ; } } Ok (()) }
};
}
