// Generated macro for path_matches_base (function)
macro_rules! Depcrate_unnecessary_struct_initializationpath_matches_base {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"path_matches_base"}
// Dependencies: {}
# [doc = " When some fields are assigned from a base struct and others individually"] # [doc = " the lint applies only if the source of the field is the same as the base."] # [doc = " This is enforced here by comparing the path of the base expression;"] # [doc = " needless to say the lint only applies if it (or whatever expression it is"] # [doc = " a reference of) actually has a path."] fn path_matches_base (path : & Path < '_ > , base : & Expr < '_ >) -> bool { let base_path = match base . kind { ExprKind :: Unary (UnOp :: Deref , base_expr) => { if let ExprKind :: Path (QPath :: Resolved (_ , base_path)) = base_expr . kind { base_path } else { return false ; } } , ExprKind :: Path (QPath :: Resolved (_ , base_path)) => base_path , _ => return false , } ; path . res == base_path . res }
};
}
