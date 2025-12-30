// Generated macro for make_binop (function)
macro_rules! Depcrate_suggmake_binop {
() => {
// Module: crate::sugg
// Provides: {"make_binop"}
// Dependencies: {}
# [doc = " Convenience wrapper around `make_assoc` and `AssocOp::Binary`."] pub fn make_binop (op : ast :: BinOpKind , lhs : & Sugg < '_ > , rhs : & Sugg < '_ >) -> Sugg < 'static > { make_assoc (AssocOp :: Binary (op) , lhs , rhs) }
};
}
