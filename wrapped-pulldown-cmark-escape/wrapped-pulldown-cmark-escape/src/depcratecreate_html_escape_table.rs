// Generated macro for create_html_escape_table (function)
macro_rules! Depcratecreate_html_escape_table {
() => {
// Module: crate
// Provides: {"create_html_escape_table"}
// Dependencies: {}
const fn create_html_escape_table (body : bool) -> [u8 ; 256] { let mut table = [0 ; 256] ; table [b'&' as usize] = 1 ; table [b'<' as usize] = 2 ; table [b'>' as usize] = 3 ; if ! body { table [b'"' as usize] = 4 ; table [b'\'' as usize] = 5 ; } table }
};
}
