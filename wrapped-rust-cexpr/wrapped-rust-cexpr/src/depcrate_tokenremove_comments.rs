// Generated macro for remove_comments (function)
macro_rules! Depcrate_tokenremove_comments {
() => {
// Module: crate::token
// Provides: {"remove_comments"}
// Dependencies: {}
# [doc = " Remove all comment tokens from a vector of tokens"] pub fn remove_comments (v : & mut Vec < Token >) -> & mut Vec < Token > { v . retain (| t | t . kind != Kind :: Comment) ; v }
};
}
