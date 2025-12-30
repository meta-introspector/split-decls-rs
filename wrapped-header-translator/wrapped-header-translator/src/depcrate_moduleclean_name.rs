// Generated macro for clean_name (function)
macro_rules! Depcrate_moduleclean_name {
() => {
// Module: crate::module
// Provides: {"clean_name"}
// Dependencies: {}
# [doc = " Some SDK files have '+' in the file name, so we change those to `_`."] pub (crate) fn clean_name (name : & str) -> String { name . replace ('+' , "_") }
};
}
