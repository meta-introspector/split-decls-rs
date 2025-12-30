// Generated macro for AssignOpKind (enum)
macro_rules! Depcrate_astAssignOpKind {
() => {
// Module: crate::ast
// Provides: {"AssignOpKind"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum AssignOpKind { # [doc = " The `+=` operator (addition)"] AddAssign , # [doc = " The `-=` operator (subtraction)"] SubAssign , # [doc = " The `*=` operator (multiplication)"] MulAssign , # [doc = " The `/=` operator (division)"] DivAssign , # [doc = " The `%=` operator (modulus)"] RemAssign , # [doc = " The `^=` operator (bitwise xor)"] BitXorAssign , # [doc = " The `&=` operator (bitwise and)"] BitAndAssign , # [doc = " The `|=` operator (bitwise or)"] BitOrAssign , # [doc = " The `<<=` operator (shift left)"] ShlAssign , # [doc = " The `>>=` operator (shift right)"] ShrAssign , }
};
}
