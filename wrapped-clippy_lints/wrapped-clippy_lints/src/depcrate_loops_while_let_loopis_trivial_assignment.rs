// Generated macro for is_trivial_assignment (function)
macro_rules! Depcrate_loops_while_let_loopis_trivial_assignment {
() => {
// Module: crate::loops::while_let_loop
// Provides: {"is_trivial_assignment"}
// Dependencies: {}
fn is_trivial_assignment (pat : & Pat < '_ > , init : & Expr < '_ >) -> bool { match (pat . kind , init . kind) { (PatKind :: Wild , _) => true , (PatKind :: Binding (BindingMode :: NONE , _ , pat_ident , None) , ExprKind :: Path (QPath :: Resolved (None , Path { segments : [init] , .. })) ,) => pat_ident . name == init . ident . name , _ => false , } }
};
}
