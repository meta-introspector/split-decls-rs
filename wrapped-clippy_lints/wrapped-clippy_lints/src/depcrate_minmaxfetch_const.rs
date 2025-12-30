// Generated macro for fetch_const (function)
macro_rules! Depcrate_minmaxfetch_const {
() => {
// Module: crate::minmax
// Provides: {"fetch_const"}
// Dependencies: {}
fn fetch_const < 'a > (cx : & LateContext < '_ > , ctxt : SyntaxContext , receiver : Option < & 'a Expr < 'a > > , args : & 'a [Expr < 'a >] , m : MinMax ,) -> Option < (MinMax , Constant , & 'a Expr < 'a >) > { let mut args = receiver . into_iter () . chain (args) ; let first_arg = args . next () ? ; let second_arg = args . next () ? ; if args . next () . is_some () { return None ; } let ecx = ConstEvalCtxt :: new (cx) ; match (ecx . eval_local (first_arg , ctxt) , ecx . eval_local (second_arg , ctxt)) { (Some (c) , None) => Some ((m , c , second_arg)) , (None , Some (c)) => Some ((m , c , first_arg)) , _ => None , } }
};
}
