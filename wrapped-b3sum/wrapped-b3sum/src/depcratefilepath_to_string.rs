// Generated macro for filepath_to_string (function)
macro_rules! Depcratefilepath_to_string {
() => {
// Module: crate
// Provides: {"filepath_to_string"}
// Dependencies: {}
fn filepath_to_string (filepath : & Path) -> FilepathString { let unicode_cow = filepath . to_string_lossy () ; let mut filepath_string = unicode_cow . to_string () ; if cfg ! (windows) { filepath_string = filepath_string . replace ('\\' , "/") ; } let mut is_escaped = false ; if filepath_string . contains (['\\' , '\n' , '\r']) { filepath_string = filepath_string . replace ('\\' , "\\\\") . replace ('\n' , "\\n") . replace ('\r' , "\\r") ; is_escaped = true ; } FilepathString { filepath_string , is_escaped , } }
};
}
