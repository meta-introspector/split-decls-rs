// Generated macro for read_file_lines (function)
macro_rules! Depcrate_ioread_file_lines {
() => {
// Module: crate::io
// Provides: {"read_file_lines"}
// Dependencies: {}
# [track_caller] pub fn read_file_lines (path : & str) -> Vec < String > { let Ok (file) = std :: fs :: File :: open (path) else { panic ! ("failed to open file `{path}`") } ; let file = std :: io :: BufReader :: new (file) ; let mut lines = vec ! [] ; for line in file . lines () { let Ok (line) = line else { panic ! ("failed to read file lines `{path}`") ; } ; lines . push (line) ; } lines }
};
}
