// Generated macro for try_simplified (function)
macro_rules! Depcratetry_simplified {
() => {
// Module: crate
// Provides: {"try_simplified"}
// Dependencies: {}
# [cfg (windows)] fn try_simplified (path : & Path) -> Option < & Path > { let mut components = path . components () ; match components . next () { Some (Component :: Prefix (p)) => match p . kind () { Prefix :: VerbatimDisk (..) => { } , _ => return None , } , _ => return None , } let stripped_path = components . as_path () ; for component in components { match component { Component :: RootDir => { } , Component :: Normal (file_name) => { if ! is_valid_filename (file_name) || is_reserved (file_name) { return None ; } } _ => return None , } ; } let path_os_str = stripped_path . as_os_str () ; if path_os_str . len () > 260 && windows_char_len (path_os_str) > 260 { return None ; } Some (stripped_path) }
};
}
