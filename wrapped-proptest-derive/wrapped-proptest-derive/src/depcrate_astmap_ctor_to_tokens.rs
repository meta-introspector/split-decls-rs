// Generated macro for map_ctor_to_tokens (function)
macro_rules! Depcrate_astmap_ctor_to_tokens {
() => {
// Module: crate::ast
// Provides: {"map_ctor_to_tokens"}
// Dependencies: {}
fn map_ctor_to_tokens (tokens : & mut TokenStream , ctors : & [Ctor] , closure : & MapClosure ,) { let ctors = NestedTuple (ctors) ; quote_append ! (tokens , _proptest :: strategy :: Strategy :: prop_map (# ctors , # closure)) ; }
};
}
