// Generated macro for impl_690 (impl)
macro_rules! Depcrate_suggimpl_690 {
() => {
// Module: crate::sugg
// Provides: {"impl_690"}
// Dependencies: {}
impl < 'a > Neg for Sugg < 'a > { type Output = Sugg < 'a > ; fn neg (self) -> Self :: Output { match self { Self :: UnOp (UnOp :: Neg , sugg) => * sugg , Self :: BinOp (AssocOp :: Cast , ..) => Sugg :: MaybeParen (format ! ("-({self})") . into ()) , _ => make_unop ("-" , self) , } } }
};
}
