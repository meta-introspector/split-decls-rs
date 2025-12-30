// Generated macro for check_inputs (function)
macro_rules! Depcrate_eta_reductioncheck_inputs {
() => {
// Module: crate::eta_reduction
// Provides: {"check_inputs"}
// Dependencies: {}
fn check_inputs (typeck : & TypeckResults < '_ > , params : & [Param < '_ >] , self_arg : Option < & Expr < '_ > > , args : & [Expr < '_ >] ,) -> bool { params . len () == self_arg . map_or (0 , | _ | 1) + args . len () && params . iter () . zip (self_arg . into_iter () . chain (args)) . all (| (p , arg) | { matches ! (p . pat . kind , PatKind :: Binding (BindingMode :: NONE , id , _ , None) if arg . res_local_id () == Some (id)) && typeck . expr_adjustments (arg) . last () . is_none_or (| a | a . target == typeck . expr_ty (arg)) }) }
};
}
