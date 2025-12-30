// Generated macro for use_4 (pub_use)
macro_rules! Depcrateuse_4 {
() => {
// Module: crate
// Provides: {"use_4"}
// Dependencies: {}
# [cfg (feature = "macros")] # [doc = " Macro to create a `static` (compile-time) [`Set`]."] # [doc = ""] # [doc = " Requires the `macros` feature."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use phf::{phf_set, Set};"] # [doc = ""] # [doc = " static MY_SET: Set<&'static str> = phf_set! {"] # [doc = "     \"hello world\","] # [doc = "     \"hola mundo\","] # [doc = " };"] # [doc = ""] # [doc = " fn main () {"] # [doc = "     assert!(MY_SET.contains(\"hello world\"));"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # OR Patterns"] # [doc = ""] # [doc = " You can use OR patterns to include multiple keys in a single entry:"] # [doc = ""] # [doc = " ```"] # [doc = " use phf::{phf_set, Set};"] # [doc = ""] # [doc = " static KEYWORDS: Set<&'static str> = phf_set! {"] # [doc = "     \"if\" | \"elif\" | \"else\","] # [doc = "     \"for\" | \"while\" | \"loop\","] # [doc = "     \"fn\" | \"function\" | \"def\","] # [doc = " };"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert!(KEYWORDS.contains(\"if\"));"] # [doc = "     assert!(KEYWORDS.contains(\"elif\"));"] # [doc = "     assert!(KEYWORDS.contains(\"else\"));"] # [doc = "     assert!(KEYWORDS.contains(\"for\"));"] # [doc = " }"] # [doc = " ```"] pub use phf_macros :: phf_set ;
};
}
