// Generated macro for associativity (function)
macro_rules! Depcrate_suggassociativity {
() => {
// Module: crate::sugg
// Provides: {"associativity"}
// Dependencies: {}
# [doc = " Returns the associativity/fixity of an operator. The difference with"] # [doc = " `AssocOp::fixity` is that an operator can be both left and right associative"] # [doc = " (such as `+`: `a + b + c == (a + b) + c == a + (b + c)`."] # [doc = ""] # [doc = " Chained `as` and explicit `:` type coercion never need inner parenthesis so"] # [doc = " they are considered"] # [doc = " associative."] # [must_use] fn associativity (op : AssocOp) -> Associativity { use ast :: BinOpKind :: { Add , And , BitAnd , BitOr , BitXor , Div , Eq , Ge , Gt , Le , Lt , Mul , Ne , Or , Rem , Shl , Shr , Sub } ; use rustc_ast :: util :: parser :: AssocOp :: { Assign , AssignOp , Binary , Cast , Range } ; match op { Assign | AssignOp (_) => Associativity :: Right , Binary (Add | BitAnd | BitOr | BitXor | And | Or | Mul) | Cast => Associativity :: Both , Binary (Div | Eq | Gt | Ge | Lt | Le | Rem | Ne | Shl | Shr | Sub) => Associativity :: Left , Range (_) => Associativity :: None , } }
};
}
