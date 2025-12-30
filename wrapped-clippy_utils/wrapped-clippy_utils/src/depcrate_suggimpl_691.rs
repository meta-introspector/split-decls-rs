// Generated macro for impl_691 (impl)
macro_rules! Depcrate_suggimpl_691 {
() => {
// Module: crate::sugg
// Provides: {"impl_691"}
// Dependencies: {}
impl < 'a > Not for Sugg < 'a > { type Output = Sugg < 'a > ; fn not (self) -> Sugg < 'a > { use AssocOp :: Binary ; use ast :: BinOpKind :: { Eq , Ge , Gt , Le , Lt , Ne } ; match self { Sugg :: BinOp (op , lhs , rhs) => { let to_op = match op { Binary (Eq) => Binary (Ne) , Binary (Ne) => Binary (Eq) , Binary (Lt) => Binary (Ge) , Binary (Ge) => Binary (Lt) , Binary (Gt) => Binary (Le) , Binary (Le) => Binary (Gt) , _ => return make_unop ("!" , Sugg :: BinOp (op , lhs , rhs)) , } ; Sugg :: BinOp (to_op , lhs , rhs) } , Sugg :: UnOp (UnOp :: Not , expr) => * expr , _ => make_unop ("!" , self) , } } }
};
}
