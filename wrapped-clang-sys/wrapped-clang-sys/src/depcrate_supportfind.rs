// Generated macro for find (function)
macro_rules! Depcrate_supportfind {
() => {
// Module: crate::support
// Provides: {"find"}
// Dependencies: {}
# [doc = " Returns the first match to the supplied glob patterns in the supplied"] # [doc = " directory if there are any matches."] fn find (directory : & Path , patterns : & [& str]) -> Option < PathBuf > { let directory = if let Some (directory) = directory . to_str () { Path :: new (& Pattern :: escape (directory)) . to_owned () } else { return None ; } ; for pattern in patterns { let pattern = directory . join (pattern) . to_string_lossy () . into_owned () ; if let Some (path) = glob :: glob (& pattern) . ok () ? . filter_map (| p | p . ok ()) . next () { if path . is_file () && is_executable (& path) . unwrap_or (false) { return Some (path) ; } } } None }
};
}
