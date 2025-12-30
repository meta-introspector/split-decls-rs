// Generated macro for make_iterator_snippet (function)
macro_rules! Depcrate_loops_utilsmake_iterator_snippet {
() => {
// Module: crate::loops::utils
// Provides: {"make_iterator_snippet"}
// Dependencies: {}
# [doc = " If `arg` was the argument to a `for` loop, return the \"cleanest\" way of writing the"] # [doc = " actual `Iterator` that the loop uses."] pub (super) fn make_iterator_snippet (cx : & LateContext < '_ > , arg : & Expr < '_ > , applicability : & mut Applicability) -> String { let impls_iterator = cx . tcx . get_diagnostic_item (sym :: Iterator) . is_some_and (| id | implements_trait (cx , cx . typeck_results () . expr_ty (arg) , id , & [])) ; if impls_iterator { format ! ("{}" , sugg :: Sugg :: hir_with_applicability (cx , arg , "_" , applicability) . maybe_paren ()) } else { let arg_ty = cx . typeck_results () . expr_ty_adjusted (arg) ; match & arg_ty . kind () { ty :: Ref (_ , inner_ty , mutbl) if has_iter_method (cx , * inner_ty) . is_some () => { let method_name = match mutbl { Mutability :: Mut => "iter_mut" , Mutability :: Not => "iter" , } ; let caller = match & arg . kind { ExprKind :: AddrOf (BorrowKind :: Ref , _ , arg_inner) => arg_inner , _ => arg , } ; format ! ("{}.{method_name}()" , sugg :: Sugg :: hir_with_applicability (cx , caller , "_" , applicability) . maybe_paren () ,) } , _ => format ! ("{}.into_iter()" , sugg :: Sugg :: hir_with_applicability (cx , arg , "_" , applicability) . maybe_paren ()) , } } }
};
}
