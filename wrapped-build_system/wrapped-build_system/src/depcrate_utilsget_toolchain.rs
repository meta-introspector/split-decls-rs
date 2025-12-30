// Generated macro for get_toolchain (function)
macro_rules! Depcrate_utilsget_toolchain {
() => {
// Module: crate::utils
// Provides: {"get_toolchain"}
// Dependencies: {}
pub fn get_toolchain () -> Result < String , String > { let content = match fs :: read_to_string ("rust-toolchain") { Ok (content) => content , Err (_) => return Err ("No `rust-toolchain` file found" . to_string ()) , } ; match content . split ('\n') . map (| line | line . trim ()) . filter (| line | ! line . is_empty ()) . filter_map (| line | { if ! line . starts_with ("channel") { return None ; } line . split ('"') . nth (1) }) . next () { Some (toolchain) => Ok (toolchain . to_string ()) , None => Err ("Couldn't find `channel` in `rust-toolchain` file" . to_string ()) , } }
};
}
