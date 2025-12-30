// Generated macro for cp_r (function)
macro_rules! Depcratecp_r {
() => {
// Module: crate
// Provides: {"cp_r"}
// Dependencies: {}
fn cp_r (src : & Path , dst : & Path) -> Result < () , String > { for f in fs :: read_dir (src) . map_err (| e | format ! ("{}: {e}" , src . display ())) ? { let f = match f { Ok (f) => f , _ => continue , } ; let path = f . path () ; let name = path . file_name () . ok_or_else (| | format ! ("bad dir {}" , src . display ())) ? ; if name . to_str () == Some (".git") { continue ; } let dst = dst . join (name) ; let ty = f . file_type () . map_err (| e | e . to_string ()) ? ; if ty . is_dir () { fs :: create_dir_all (& dst) . map_err (| e | e . to_string ()) ? ; cp_r (& path , & dst) ? ; } else if ty . is_symlink () && path . iter () . any (| p | p == "cloudflare-quiche") { continue ; } else { let _ = fs :: remove_file (& dst) ; if let Err (e) = fs :: copy (& path , & dst) { return Err (format ! ("failed to copy '{}' to '{}': {e}" , path . display () , dst . display ())) ; } } } Ok (()) }
};
}
