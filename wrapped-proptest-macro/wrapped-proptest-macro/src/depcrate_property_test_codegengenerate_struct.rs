// Generated macro for generate_struct (function)
macro_rules! Depcrate_property_test_codegengenerate_struct {
() => {
// Module: crate::property_test::codegen
// Provides: {"generate_struct"}
// Dependencies: {}
# [doc = " Generate the inner struct that represents the arguments of the function"] fn generate_struct (fn_name : & Ident , args : & [Argument]) -> TokenStream { let struct_name = struct_name (fn_name) ; let fields = args . iter () . enumerate () . map (| (index , arg) | { let field_name = nth_field_name (args , index) ; let ty = & arg . pat_ty . ty ; quote ! { # field_name : # ty , } }) ; quote ! { # [derive (Debug)] struct # struct_name { # (# fields) * } } }
};
}
