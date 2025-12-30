// Generated macro for is_mutated_static (function)
macro_rules! Depcrate_functions_must_useis_mutated_static {
() => {
// Module: crate::functions::must_use
// Provides: {"is_mutated_static"}
// Dependencies: {}
fn is_mutated_static (e : & hir :: Expr < '_ >) -> bool { use hir :: ExprKind :: { Field , Index , Path } ; match e . kind { Path (QPath :: Resolved (_ , path)) => ! matches ! (path . res , Res :: Local (_)) , Path (_) => true , Field (inner , _) | Index (inner , _ , _) => is_mutated_static (inner) , _ => false , } }
};
}
