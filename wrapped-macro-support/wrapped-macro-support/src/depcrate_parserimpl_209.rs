// Generated macro for impl_209 (impl)
macro_rules! Depcrate_parserimpl_209 {
() => {
// Module: crate::parser
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a > NumericValue < 'a > { fn from_expr (expr : & 'a syn :: Expr) -> Option < Self > { match get_expr (expr) { syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Int (int_lit) , .. }) => Some (Self { negative : false , base10_digits : int_lit . base10_digits () , }) , syn :: Expr :: Unary (syn :: ExprUnary { op : syn :: UnOp :: Neg (_) , expr , .. }) => Self :: from_expr (expr) . map (| n | n . neg ()) , _ => None , } } fn parse (& self) -> Option < i64 > { let mut value = self . base10_digits . parse :: < i64 > () . ok () ? ; if self . negative { value = - value ; } Some (value) } fn neg (self) -> Self { Self { negative : ! self . negative , base10_digits : self . base10_digits , } } }
};
}
