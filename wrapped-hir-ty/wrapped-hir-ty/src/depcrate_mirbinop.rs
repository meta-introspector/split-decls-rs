// Generated macro for BinOp (enum)
macro_rules! Depcrate_mirBinOp {
() => {
// Module: crate::mir
// Provides: {"BinOp"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] pub enum BinOp { # [doc = " The `+` operator (addition)"] Add , # [doc = " The `-` operator (subtraction)"] Sub , # [doc = " The `*` operator (multiplication)"] Mul , # [doc = " The `/` operator (division)"] # [doc = ""] # [doc = " Division by zero is UB, because the compiler should have inserted checks"] # [doc = " prior to this."] Div , # [doc = " The `%` operator (modulus)"] # [doc = ""] # [doc = " Using zero as the modulus (second operand) is UB, because the compiler"] # [doc = " should have inserted checks prior to this."] Rem , # [doc = " The `^` operator (bitwise xor)"] BitXor , # [doc = " The `&` operator (bitwise and)"] BitAnd , # [doc = " The `|` operator (bitwise or)"] BitOr , # [doc = " The `<<` operator (shift left)"] # [doc = ""] # [doc = " The offset is truncated to the size of the first operand before shifting."] Shl , # [doc = " The `>>` operator (shift right)"] # [doc = ""] # [doc = " The offset is truncated to the size of the first operand before shifting."] Shr , # [doc = " The `==` operator (equality)"] Eq , # [doc = " The `<` operator (less than)"] Lt , # [doc = " The `<=` operator (less than or equal to)"] Le , # [doc = " The `!=` operator (not equal to)"] Ne , # [doc = " The `>=` operator (greater than or equal to)"] Ge , # [doc = " The `>` operator (greater than)"] Gt , # [doc = " The `ptr.offset` operator"] Offset , }
};
}
