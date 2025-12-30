// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
fn test (args : & str) { let mut expand = vec ! ["--no-comment" , "--in" , "default" , "--flat"] ; expand . extend (args . split_whitespace ()) ; _ = bindgen (expand) ; }
};
}
