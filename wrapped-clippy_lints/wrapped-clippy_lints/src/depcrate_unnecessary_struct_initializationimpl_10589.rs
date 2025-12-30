// Generated macro for impl_10589 (impl)
macro_rules! Depcrate_unnecessary_struct_initializationimpl_10589 {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"impl_10589"}
// Dependencies: {}
impl LateLintPass < '_ > for UnnecessaryStruct { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { let ExprKind :: Struct (_ , fields , base) = expr . kind else { return ; } ; if expr . span . from_expansion () { return ; } let field_path = same_path_in_all_fields (cx , expr , fields) ; let sugg = match (field_path , base) { (Some (& path) , StructTailExpr :: None | StructTailExpr :: DefaultFields (_)) => { path . span } , (Some (path) , StructTailExpr :: Base (base)) if base_is_suitable (cx , expr , base) && path_matches_base (path , base) => { base . span } , (None , StructTailExpr :: Base (base)) if fields . is_empty () && base_is_suitable (cx , expr , base) => { base . span } , _ => return , } ; span_lint_and_sugg (cx , UNNECESSARY_STRUCT_INITIALIZATION , expr . span , "unnecessary struct building" , "replace with" , snippet (cx , sugg , "..") . into_owned () , rustc_errors :: Applicability :: MachineApplicable ,) ; } }
};
}
