// Generated macro for path_relative_from (function)
macro_rules! Depcrate_runtimepath_relative_from {
() => {
// Module: crate::runtime
// Provides: {"path_relative_from"}
// Dependencies: {}
# [doc = " Computes a relative path from `base` to `path`, returning a path with `../` components"] # [doc = " if necessary."] # [doc = ""] # [doc = " This function is vendored from the old Rust standard library implementation"] # [doc = " (pre-1.0, removed in RFC 474) and is distributed under the same terms as the"] # [doc = " Rust project (MIT/Apache-2.0 dual license)."] # [doc = ""] # [doc = " Unlike `Path::strip_prefix`, this function can handle cases where `path` is not"] # [doc = " a descendant of `base`, making it suitable for finding relative paths between"] # [doc = " arbitrary directories (e.g., between sibling directories in a workspace)."] fn path_relative_from (path : & Path , base : & Path) -> Option < PathBuf > { use std :: path :: Component ; if path . is_absolute () != base . is_absolute () { if path . is_absolute () { Some (PathBuf :: from (path)) } else { None } } else { let mut ita = path . components () ; let mut itb = base . components () ; let mut comps : Vec < Component > = vec ! [] ; loop { match (ita . next () , itb . next ()) { (None , None) => break , (Some (a) , None) => { comps . push (a) ; comps . extend (ita . by_ref ()) ; break ; } (None , _) => comps . push (Component :: ParentDir) , (Some (a) , Some (b)) if comps . is_empty () && a == b => { } (Some (a) , Some (_b)) => { comps . push (Component :: ParentDir) ; for _ in itb { comps . push (Component :: ParentDir) ; } comps . push (a) ; comps . extend (ita . by_ref ()) ; break ; } } } Some (comps . iter () . map (| c | c . as_os_str ()) . collect ()) } }
};
}
