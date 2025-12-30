// Generated macro for impl_192 (impl)
macro_rules! Depcrate_unsorted_clippy_utils_pathsimpl_192 {
() => {
// Module: crate::unsorted_clippy_utils_paths
// Provides: {"impl_192"}
// Dependencies: {}
impl EarlyLintPass for UnsortedClippyUtilsPaths { fn check_crate (& mut self , cx : & EarlyContext < '_ > , krate : & Crate) { if let Some (utils) = krate . items . iter () . find (| item | item . kind . ident () . is_some_and (| i | i . name == sym :: utils)) && let ItemKind :: Mod (_ , _ , ModKind :: Loaded (ref items , ..)) = utils . kind && let Some (paths) = items . iter () . find (| item | item . kind . ident () . is_some_and (| i | i . name == sym :: paths)) && let ItemKind :: Mod (_ , _ , ModKind :: Loaded (ref items , ..)) = paths . kind { let mut last_name : Option < String > = None ; for item in items { let name = item . kind . ident () . expect ("const items have idents") . to_string () ; if let Some (last_name) = last_name && * last_name > * name { span_lint (cx , UNSORTED_CLIPPY_UTILS_PATHS , item . span , "this constant should be before the previous constant due to lexical \
                                         ordering" ,) ; } last_name = Some (name) ; } } } }
};
}
