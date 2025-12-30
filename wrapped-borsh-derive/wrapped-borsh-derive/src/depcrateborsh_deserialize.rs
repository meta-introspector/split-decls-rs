// Generated macro for borsh_deserialize (function)
macro_rules! Depcrateborsh_deserialize {
() => {
// Module: crate
// Provides: {"borsh_deserialize"}
// Dependencies: {}
# [doc = " ---"] # [doc = ""] # [doc = " moved to docs of **Derive Macro** `BorshDeserialize` in `borsh` crate"] # [proc_macro_derive (BorshDeserialize , attributes (borsh))] pub fn borsh_deserialize (input : TokenStream) -> TokenStream { let cratename = match check_attrs_get_cratename (& input) { Ok (cratename) => cratename , Err (err) => { return err . to_compile_error () . into () ; } } ; let res = if let Ok (input) = syn :: parse :: < ItemStruct > (input . clone ()) { deserialize :: structs :: process (& input , cratename) } else if let Ok (input) = syn :: parse :: < ItemEnum > (input . clone ()) { deserialize :: enums :: process (& input , cratename) } else if let Ok (input) = syn :: parse :: < ItemUnion > (input) { deserialize :: unions :: process (& input , cratename) } else { unreachable ! () } ; TokenStream :: from (match res { Ok (res) => res , Err (err) => err . to_compile_error () , }) }
};
}
