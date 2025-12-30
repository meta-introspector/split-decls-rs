// Generated macro for extract_msrv_attr (macro)
macro_rules! Depcrateextract_msrv_attr {
() => {
// Module: crate
// Provides: {"extract_msrv_attr"}
// Dependencies: {}
# [macro_export] macro_rules ! extract_msrv_attr { () => { fn check_attributes (& mut self , cx : & rustc_lint :: EarlyContext <'_ >, attrs : & [rustc_ast :: ast :: Attribute]) { let sess = rustc_lint :: LintContext :: sess (cx) ; self . msrv . check_attributes (sess , attrs) ; } fn check_attributes_post (& mut self , cx : & rustc_lint :: EarlyContext <'_ >, attrs : & [rustc_ast :: ast :: Attribute]) { let sess = rustc_lint :: LintContext :: sess (cx) ; self . msrv . check_attributes_post (sess , attrs) ; } } ; }
};
}
