// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_foldcheck {
() => {
// Module: crate::methods::unnecessary_fold
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , init : & hir :: Expr < '_ > , acc : & hir :: Expr < '_ > , fold_span : Span ,) { if ! cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { return ; } if let hir :: ExprKind :: Lit (lit) = init . kind { match lit . node { ast :: LitKind :: Bool (false) => { check_fold_with_op (cx , expr , acc , fold_span , hir :: BinOpKind :: Or , Replacement { method_name : "any" , has_args : true , has_generic_return : false , } ,) ; } , ast :: LitKind :: Bool (true) => { check_fold_with_op (cx , expr , acc , fold_span , hir :: BinOpKind :: And , Replacement { method_name : "all" , has_args : true , has_generic_return : false , } ,) ; } , ast :: LitKind :: Int (Pu128 (0) , _) => { check_fold_with_op (cx , expr , acc , fold_span , hir :: BinOpKind :: Add , Replacement { method_name : "sum" , has_args : false , has_generic_return : needs_turbofish (cx , expr) , } ,) ; } , ast :: LitKind :: Int (Pu128 (1) , _) => { check_fold_with_op (cx , expr , acc , fold_span , hir :: BinOpKind :: Mul , Replacement { method_name : "product" , has_args : false , has_generic_return : needs_turbofish (cx , expr) , } ,) ; } , _ => () , } } }
};
}
