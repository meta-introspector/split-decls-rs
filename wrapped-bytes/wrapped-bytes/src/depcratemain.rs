// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let word = "hello world!" ; let mut buf = Cursor :: new (word . as_bytes ()) ; let compact_str = CompactString :: from_utf8_buf (& mut buf) . expect ("valid utf-8") ; assert_eq ! (compact_str , word) ; println ! ("{}" , compact_str) ; }
};
}
