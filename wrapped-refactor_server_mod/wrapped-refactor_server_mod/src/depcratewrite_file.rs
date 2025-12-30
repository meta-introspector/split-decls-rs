// Generated macro for write_file (function)
macro_rules! Depcratewrite_file {
() => {
// Module: crate
// Provides: {"write_file"}
// Dependencies: {}
fn write_file (path : & str , file : & File) -> Result < () > { let formatted_code = prettyplease :: unparse (file) ; fs :: write (path , formatted_code) . with_context (| | format ! ("Failed to write file: {}" , path)) }
};
}
