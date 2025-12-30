// Generated macro for mangle_name (function)
macro_rules! Depcrate_declaremangle_name {
() => {
// Module: crate::declare
// Provides: {"mangle_name"}
// Dependencies: {}
# [cfg (not (feature = "master"))] fn mangle_name (name : & str) -> String { name . replace (| char : char | { if ! char . is_alphanumeric () && char != '_' { debug_assert ! ("$.*" . contains (char) , "Unsupported char in function name {}: {}" , name , char) ; true } else { false } } , "_" ,) }
};
}
