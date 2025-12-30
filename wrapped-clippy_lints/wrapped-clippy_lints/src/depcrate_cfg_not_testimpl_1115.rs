// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_cfg_not_testimpl_1115 {
() => {
// Module: crate::cfg_not_test
// Provides: {"impl_1115"}
// Dependencies: {}
impl EarlyLintPass for CfgNotTest { fn check_attribute (& mut self , cx : & EarlyContext < '_ > , attr : & rustc_ast :: Attribute) { if attr . has_name (rustc_span :: sym :: cfg_trace) && contains_not_test (attr . meta_item_list () . as_deref () , false) { span_lint_and_then (cx , CFG_NOT_TEST , attr . span , "code is excluded from test builds" , | diag | { diag . help ("consider not excluding any code from test builds") ; diag . note_once ("this could increase code coverage despite not actually being tested") ; } ,) ; } } }
};
}
