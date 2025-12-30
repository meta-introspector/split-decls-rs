// Generated macro for make_filename_safe (function)
macro_rules! Depcrate_reportmake_filename_safe {
() => {
// Module: crate::report
// Provides: {"make_filename_safe"}
// Dependencies: {}
pub fn make_filename_safe (string : & str) -> String { let mut string = string . replace (& ['?' , '"' , '/' , '\\' , '*' , '<' , '>' , ':' , '|' , '^'] [..] , "_" ,) ; truncate_to_character_boundary (& mut string , MAX_DIRECTORY_NAME_LEN) ; if cfg ! (target_os = "windows") { { string = string . trim_end () . to_lowercase () ; } } string }
};
}
