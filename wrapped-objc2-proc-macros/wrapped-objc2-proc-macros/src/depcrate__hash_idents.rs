// Generated macro for __hash_idents (function)
macro_rules! Depcrate__hash_idents {
() => {
// Module: crate
// Provides: {"__hash_idents"}
// Dependencies: {}
# [doc = " Creates a hash from the input and source code locations in the provided"] # [doc = " idents."] # [doc = ""] # [doc = " This hash is not guaranteed to be stable across compiler versions."] # [doc = ""] # [doc = " Tests are in `objc2/src/__macros/hash_idents.rs`."] # [proc_macro] # [doc (hidden)] pub fn __hash_idents (input : TokenStream) -> TokenStream { let mut hasher = std :: collections :: hash_map :: DefaultHasher :: new () ; for ident in get_idents (input) { ident . to_string () . hash (& mut hasher) ; format ! ("{:?}" , ident . span ()) . hash (& mut hasher) ; } let s = format ! ("{:016x}" , hasher . finish ()) ; TokenTree :: Literal (Literal :: string (& s)) . into () }
};
}
