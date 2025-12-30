// Generated macro for impl_10565 (impl)
macro_rules! Depcrate_unnecessary_self_importsimpl_10565 {
() => {
// Module: crate::unnecessary_self_imports
// Provides: {"impl_10565"}
// Dependencies: {}
impl EarlyLintPass for UnnecessarySelfImports { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: Use (use_tree) = & item . kind && let UseTreeKind :: Nested { items , .. } = & use_tree . kind && let [(self_tree , _)] = & * * items && let [self_seg] = & * self_tree . prefix . segments && self_seg . ident . name == kw :: SelfLower && let Some (last_segment) = use_tree . prefix . segments . last () { span_lint_and_then (cx , UNNECESSARY_SELF_IMPORTS , item . span , "import ending with `::{self}`" , | diag | { diag . span_suggestion (last_segment . span () . with_hi (item . span . hi ()) , "consider omitting `::{self}`" , format ! ("{}{};" , last_segment . ident , if let UseTreeKind :: Simple (Some (alias)) = self_tree . kind { format ! (" as {alias}") } else { String :: new () } ,) , Applicability :: MaybeIncorrect ,) ; diag . note ("this will slightly change semantics; any non-module items at the same path will also be imported") ; } ,) ; } } }
};
}
