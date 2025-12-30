// Generated macro for FoldExpr (enum)
macro_rules! Depcrate_astFoldExpr {
() => {
// Module: crate::ast
// Provides: {"FoldExpr"}
// Dependencies: {}
# [doc = " The fold expressions."] # [doc = ""] # [doc = " These are not separate productions in the grammar but our code is cleaner"] # [doc = " if we handle them all together."] # [doc = ""] # [doc = " <expression>  ::= ..."] # [doc = "               ::= fl <binary operator-name> <expression>       # (... operator expression), unary left fold"] # [doc = "               ::= fr <binary operator-name> <expression>       # (expression operator ...), unary right fold"] # [doc = "               ::= fL <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary left fold"] # [doc = "               ::= fR <binary operator-name> <expression> <expression> # (expression operator ... operator expression), binary right fold"] # [doc = "               ::= ..."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum FoldExpr { # [doc = " (...+<expr>)"] UnaryLeft (SimpleOperatorName , Box < Expression >) , # [doc = " (<expr>+...)"] UnaryRight (SimpleOperatorName , Box < Expression >) , # [doc = " (<expr1>+...+<expr2>)"] BinaryLeft (SimpleOperatorName , Box < Expression > , Box < Expression >) , # [doc = " (<expr1>+...+<expr2>)"] BinaryRight (SimpleOperatorName , Box < Expression > , Box < Expression >) , }
};
}
