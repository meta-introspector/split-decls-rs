// Generated macro for make_config (function)
macro_rules! Depcrate_property_test_codegen_test_bodymake_config {
() => {
// Module: crate::property_test::codegen::test_body
// Provides: {"make_config"}
// Dependencies: {}
fn make_config (config : Option < & Expr >) -> TokenStream { let trailing = match config { None => quote ! { :: proptest :: test_runner :: Config :: default () } , Some (config) => config . to_token_stream () , } ; quote ! { let config = :: proptest :: test_runner :: Config { test_name : Some (concat ! (module_path ! () , "::" , stringify ! ($ test_name))) , source_file : Some (file ! ()) , ..# trailing } ; } }
};
}
