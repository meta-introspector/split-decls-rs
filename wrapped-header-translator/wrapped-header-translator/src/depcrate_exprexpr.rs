// Generated macro for Expr (enum)
macro_rules! Depcrate_exprExpr {
() => {
// Module: crate::expr
// Provides: {"Expr"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub enum Expr { Signed (i64) , Unsigned (u64) , Float (f64) , MacroInvocation { id : ItemIdentifier , is_function_like : bool , evaluated : Option < Box < Expr > > , } , Enum { id : ItemIdentifier , # [doc = " The (renamed) Rust name of the variant."] variant : String , ty : Ty , attrs : HashSet < UnexposedAttr > , } , Const { id : ItemIdentifier , ty : Ty , } , Var { id : ItemIdentifier , ty : Ty , } , Tokens (Vec < Token >) , }
};
}
