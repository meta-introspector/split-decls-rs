// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_crate_in_macro_defimpl_1304 {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"impl_1304"}
// Dependencies: {}
impl EarlyLintPass for CrateInMacroDef { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { if let ItemKind :: MacroDef (_ , macro_def) = & item . kind && item . attrs . iter () . any (is_macro_export) && let Some (span) = contains_unhygienic_crate_reference (& macro_def . body . tokens) { span_lint_and_sugg (cx , CRATE_IN_MACRO_DEF , span , "`crate` references the macro call's crate" , "to reference the macro definition's crate, use" , String :: from ("$crate") , Applicability :: MachineApplicable ,) ; } } }
};
}
