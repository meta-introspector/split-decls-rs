// Generated macro for UnOp (enum)
macro_rules! Depcrate_astUnOp {
() => {
// Module: crate::ast
// Provides: {"UnOp"}
// Dependencies: {}
# [doc = " Unary operator."] # [doc = ""] # [doc = " Note that `&data` is not an operator, it's an `AddrOf` expression."] # [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum UnOp { # [doc = " The `*` operator for dereferencing"] Deref , # [doc = " The `!` operator for logical inversion"] Not , # [doc = " The `-` operator for negation"] Neg , }
};
}
