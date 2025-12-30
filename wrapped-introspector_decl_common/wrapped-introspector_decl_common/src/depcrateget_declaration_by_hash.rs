// Generated macro for get_declaration_by_hash (function)
macro_rules! Depcrateget_declaration_by_hash {
() => {
// Module: crate
// Provides: {"get_declaration_by_hash"}
// Dependencies: {}
pub fn get_declaration_by_hash (hash : & str) -> Option < DeclInfo > { DECL_REGISTRY . lock () . ok () ? . by_hash . get (hash) . and_then (| & idx | { DECL_REGISTRY . lock () . ok () ? . declarations . get (idx) . cloned () }) }
};
}
