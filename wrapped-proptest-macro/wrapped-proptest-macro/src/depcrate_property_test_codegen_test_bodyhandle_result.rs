// Generated macro for handle_result (function)
macro_rules! Depcrate_property_test_codegen_test_bodyhandle_result {
() => {
// Module: crate::property_test::codegen::test_body
// Provides: {"handle_result"}
// Dependencies: {}
# [doc = " rough heuristic for whether we should use result-style syntax - if the function returns either"] # [doc = " nothing (i.e. `()`) or an empty tuple, it will be non-result handling, otherwise it uses"] # [doc = " result-style handling"] # [doc = ""] # [doc = " Note, this won't catch cases like `type Foo = ();`, since type information isn't available yet,"] # [doc = " it's just looking for the syntax `fn foo() {}` or `fn foo() -> () {}`"] fn handle_result (ret_ty : & ReturnType) -> TokenStream { let default_body = | | quote ! { let _ = result ; Ok (()) } ; let result_body = | | quote ! { result } ; match ret_ty { ReturnType :: Default => default_body () , ReturnType :: Type (_ , ty) => match ty . as_ref () { Type :: Tuple (TypeTuple { elems , .. }) if elems . is_empty () => { default_body () } _ => result_body () , } , } }
};
}
