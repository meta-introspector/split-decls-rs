// Generated macro for impl_9531 (impl)
macro_rules! Depcrate_single_component_path_importsimpl_9531 {
() => {
// Module: crate::single_component_path_imports
// Provides: {"impl_9531"}
// Dependencies: {}
impl EarlyLintPass for SingleComponentPathImports { fn check_crate (& mut self , cx : & EarlyContext < '_ > , krate : & Crate) { if cx . sess () . opts . edition < Edition :: Edition2018 { return ; } self . check_mod (& krate . items) ; } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { for SingleUse { span , can_suggest , .. } in self . found . remove (& item . id) . into_iter () . flatten () { if can_suggest { span_lint_and_sugg (cx , SINGLE_COMPONENT_PATH_IMPORTS , span , "this import is redundant" , "remove it entirely" , String :: new () , Applicability :: MachineApplicable ,) ; } else { span_lint_and_help (cx , SINGLE_COMPONENT_PATH_IMPORTS , span , "this import is redundant" , None , "remove this import" ,) ; } } } }
};
}
