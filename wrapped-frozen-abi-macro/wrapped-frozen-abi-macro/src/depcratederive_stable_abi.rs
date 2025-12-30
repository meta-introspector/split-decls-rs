// Generated macro for derive_stable_abi (function)
macro_rules! Depcratederive_stable_abi {
() => {
// Module: crate
// Provides: {"derive_stable_abi"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] # [proc_macro_derive (StableAbi)] pub fn derive_stable_abi (item : TokenStream) -> TokenStream { use { quote :: quote , syn :: { parse_macro_input , Error , Item } , } ; let item = parse_macro_input ! (item as Item) ; let ident = match item { Item :: Struct (ref s) => & s . ident , Item :: Enum (ref e) => & e . ident , Item :: Type (ref t) => & t . ident , _ => { return Error :: new_spanned (item , "StableAbi can only be derived for struct, enum, or type alias" ,) . to_compile_error () . into () ; } } ; let expanded = quote ! { # [automatically_derived] impl :: solana_frozen_abi :: stable_abi :: StableAbi for # ident { } } ; expanded . into () }
};
}
