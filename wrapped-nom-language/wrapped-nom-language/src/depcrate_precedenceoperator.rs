// Generated macro for Operator (enum)
macro_rules! Depcrate_precedenceOperator {
() => {
// Module: crate::precedence
// Provides: {"Operator"}
// Dependencies: {}
# [doc = " Element for operator stack."] enum Operator < P1 , P2 , P3 , Q : Ord + Copy > { Prefix (P1 , Q) , Postfix (P2 , Q) , Binary (P3 , Q , Assoc) , }
};
}
