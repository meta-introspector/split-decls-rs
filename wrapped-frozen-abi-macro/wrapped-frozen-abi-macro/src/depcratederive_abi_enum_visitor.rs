// Generated macro for derive_abi_enum_visitor (function)
macro_rules! Depcratederive_abi_enum_visitor {
() => {
// Module: crate
// Provides: {"derive_abi_enum_visitor"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] # [proc_macro_derive (AbiEnumVisitor)] pub fn derive_abi_enum_visitor (item : TokenStream) -> TokenStream { let item = parse_macro_input ! (item as Item) ; match item { Item :: Enum (input) => do_derive_abi_enum_visitor (input) , _ => Error :: new_spanned (item , "AbiEnumVisitor not applicable; only for enum") . to_compile_error () . into () , } }
};
}
