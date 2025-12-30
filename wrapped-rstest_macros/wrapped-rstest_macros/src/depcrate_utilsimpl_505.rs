// Generated macro for impl_505 (impl)
macro_rules! Depcrate_utilsimpl_505 {
() => {
// Module: crate::utils
// Provides: {"impl_505"}
// Dependencies: {}
impl < E : AsRef < Expr > > IsLiteralExpression for E { fn is_literal (& self) -> bool { matches ! (self . as_ref () , Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (_) , .. })) } }
};
}
