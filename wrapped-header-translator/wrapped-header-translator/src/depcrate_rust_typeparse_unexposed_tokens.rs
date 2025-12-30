// Generated macro for parse_unexposed_tokens (function)
macro_rules! Depcrate_rust_typeparse_unexposed_tokens {
() => {
// Module: crate::rust_type
// Provides: {"parse_unexposed_tokens"}
// Dependencies: {}
# [doc = " Strip macros from unexposed types."] # [doc = ""] # [doc = " These appear in newer clang versions."] # [doc = " - NS_SWIFT_NAME"] # [doc = " - NS_SWIFT_UNAVAILABLE"] # [doc = " - NS_REFINED_FOR_SWIFT"] # [doc = " - ..."] fn parse_unexposed_tokens (s : & str) -> (String , Option < UnexposedAttr >) { let tokens = TokenStream :: from_str (s) . expect ("parse attributed name") ; let mut iter = tokens . into_iter () . peekable () ; let attr = if let Some (TokenTree :: Ident (ident)) = iter . peek () { let ident = ident . to_string () ; if let Ok (attr) = UnexposedAttr :: from_name (& ident , | | { if let Some (TokenTree :: Group (_)) = iter . peek () { let Some (TokenTree :: Group (group)) = iter . next () else { unreachable ! () ; } ; group . stream () } else { trace ! (? ident , "expected group in macro") ; TokenStream :: new () } }) { iter . next () ; attr } else { None } } else { None } ; (TokenStream :: from_iter (iter) . to_string () , attr) }
};
}
