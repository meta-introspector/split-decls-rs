// Generated macro for impl_648 (impl)
macro_rules! Depcrate_suggimpl_648 {
() => {
// Module: crate::sugg
// Provides: {"impl_648"}
// Dependencies: {}
impl Display for Sugg < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match self { Sugg :: NonParen (s) | Sugg :: MaybeParen (s) => s . fmt (f) , Sugg :: BinOp (op , lhs , rhs) => binop_to_string (* op , lhs , rhs) . fmt (f) , Sugg :: UnOp (op , inner) => write ! (f , "{}{}" , op . as_str () , inner . clone () . maybe_inner_paren ()) , } } }
};
}
