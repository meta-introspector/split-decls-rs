// Generated macro for borsh_schema (function)
macro_rules! Depcrateborsh_schema {
() => {
// Module: crate
// Provides: {"borsh_schema"}
// Dependencies: {}
# [doc = " ---"] # [doc = ""] # [doc = " moved to docs of **Derive Macro** `BorshSchema` in `borsh` crate"] # [cfg (feature = "schema")] # [proc_macro_derive (BorshSchema , attributes (borsh))] pub fn borsh_schema (input : TokenStream) -> TokenStream { let cratename = match check_attrs_get_cratename (& input) { Ok (cratename) => cratename , Err (err) => { return err . to_compile_error () . into () ; } } ; let res = if let Ok (input) = syn :: parse :: < ItemStruct > (input . clone ()) { schema :: structs :: process (& input , cratename) } else if let Ok (input) = syn :: parse :: < ItemEnum > (input . clone ()) { schema :: enums :: process (& input , cratename) } else if syn :: parse :: < ItemUnion > (input) . is_ok () { Err (syn :: Error :: new (Span :: call_site () , "Borsh schema does not support unions yet." ,)) } else { unreachable ! () } ; TokenStream :: from (match res { Ok (res) => res , Err (err) => err . to_compile_error () , }) }
};
}
