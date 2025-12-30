// Generated macro for rev_concat (function)
macro_rules! Depcrate_parserrev_concat {
() => {
// Module: crate::parser
// Provides: {"rev_concat"}
// Dependencies: {}
fn rev_concat (mut exprs : Vec < Expr >) -> Expr { if exprs . len () == 0 { Expr :: Empty } else if exprs . len () == 1 { exprs . pop () . unwrap () } else { exprs . reverse () ; Expr :: Concat (exprs) } }
};
}
