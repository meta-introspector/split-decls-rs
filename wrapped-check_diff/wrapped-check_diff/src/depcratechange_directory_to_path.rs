// Generated macro for change_directory_to_path (function)
macro_rules! Depcratechange_directory_to_path {
() => {
// Module: crate
// Provides: {"change_directory_to_path"}
// Dependencies: {}
pub fn change_directory_to_path (dest : & Path) -> io :: Result < () > { let dest_path = Path :: new (& dest) ; env :: set_current_dir (& dest_path) ? ; info ! ("Current directory: {}" , env :: current_dir () . unwrap () . display ()) ; return Ok (()) ; }
};
}
