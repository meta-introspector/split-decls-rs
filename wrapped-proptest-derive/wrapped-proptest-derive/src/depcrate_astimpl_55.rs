// Generated macro for impl_55 (impl)
macro_rules! Depcrate_astimpl_55 {
() => {
// Module: crate::ast
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a , T : ToTokens > ToTokens for NestedTuple < 'a , T > { fn to_tokens (& self , tokens : & mut TokenStream) { let NestedTuple (elems) = self ; if elems . is_empty () { quote_append ! (tokens , ()) ; } else if let [x] = elems { x . to_tokens (tokens) ; } else { let chunks = elems . chunks (NESTED_TUPLE_CHUNK_SIZE) ; Recurse (& chunks) . to_tokens (tokens) ; } struct Recurse < 'a , T : ToTokens > (& 'a :: std :: slice :: Chunks < 'a , T >) ; impl < 'a , T : ToTokens > ToTokens for Recurse < 'a , T > { fn to_tokens (& self , tokens : & mut TokenStream) { let mut chunks = self . 0 . clone () ; if let Some (head) = chunks . next () { if let [c] = head { quote_append ! (tokens , # c) ; } else { let tail = Recurse (& chunks) ; quote_append ! (tokens , (# (# head ,) * # tail)) ; } } } } } }
};
}
