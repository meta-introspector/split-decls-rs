// Generated macro for BinOpKind (enum)
macro_rules! Depcrate_astBinOpKind {
() => {
// Module: crate::ast
// Provides: {"BinOpKind"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum BinOpKind { # [doc = " The `+` operator (addition)"] Add , # [doc = " The `-` operator (subtraction)"] Sub , # [doc = " The `*` operator (multiplication)"] Mul , # [doc = " The `/` operator (division)"] Div , # [doc = " The `%` operator (modulus)"] Rem , # [doc = " The `&&` operator (logical and)"] And , # [doc = " The `||` operator (logical or)"] Or , # [doc = " The `^` operator (bitwise xor)"] BitXor , # [doc = " The `&` operator (bitwise and)"] BitAnd , # [doc = " The `|` operator (bitwise or)"] BitOr , # [doc = " The `<<` operator (shift left)"] Shl , # [doc = " The `>>` operator (shift right)"] Shr , # [doc = " The `==` operator (equality)"] Eq , # [doc = " The `<` operator (less than)"] Lt , # [doc = " The `<=` operator (less than or equal to)"] Le , # [doc = " The `!=` operator (not equal to)"] Ne , # [doc = " The `>=` operator (greater than or equal to)"] Ge , # [doc = " The `>` operator (greater than)"] Gt , }
};
}
