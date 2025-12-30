// Generated macro for derive_abi_sample (function)
macro_rules! Depcratederive_abi_sample {
() => {
// Module: crate
// Provides: {"derive_abi_sample"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] # [proc_macro_derive (AbiExample)] pub fn derive_abi_sample (item : TokenStream) -> TokenStream { let item = parse_macro_input ! (item as Item) ; match item { Item :: Struct (input) => derive_abi_sample_struct_type (input) , Item :: Enum (input) => derive_abi_sample_enum_type (input) , _ => Error :: new_spanned (item , "AbiSample isn't applicable; only for struct and enum") . to_compile_error () . into () , } }
};
}
