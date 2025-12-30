// Generated macro for frozen_abi_struct_type (function)
macro_rules! Depcratefrozen_abi_struct_type {
() => {
// Module: crate
// Provides: {"frozen_abi_struct_type"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn frozen_abi_struct_type (input : ItemStruct , expected_api_digest : & str , expected_abi_digest : Option < & str > ,) -> TokenStream { let type_name = & input . ident ; let test = quote_for_test (& test_mod_name (type_name) , type_name , expected_api_digest , expected_abi_digest ,) ; let result = quote ! { # input # test } ; result . into () }
};
}
