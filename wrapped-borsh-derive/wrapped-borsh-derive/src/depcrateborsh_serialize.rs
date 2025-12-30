// Generated macro for borsh_serialize (function)
macro_rules! Depcrateborsh_serialize {
() => {
// Module: crate
// Provides: {"borsh_serialize"}
// Dependencies: {}
# [doc = " ---"] # [doc = ""] # [doc = " moved to docs of **Derive Macro** `BorshSerialize` in `borsh` crate"] # [proc_macro_derive (BorshSerialize , attributes (borsh))] pub fn borsh_serialize (input : TokenStream) -> TokenStream { let cratename = match check_attrs_get_cratename (& input) { Ok (cratename) => cratename , Err (err) => { return err . to_compile_error () . into () ; } } ; let res = if let Ok (input) = syn :: parse :: < ItemStruct > (input . clone ()) { serialize :: structs :: process (& input , cratename) } else if let Ok (input) = syn :: parse :: < ItemEnum > (input . clone ()) { serialize :: enums :: process (& input , cratename) } else if let Ok (input) = syn :: parse :: < ItemUnion > (input) { serialize :: unions :: process (& input , cratename) } else { unreachable ! () } ; TokenStream :: from (match res { Ok (res) => res , Err (err) => err . to_compile_error () , }) }
};
}
