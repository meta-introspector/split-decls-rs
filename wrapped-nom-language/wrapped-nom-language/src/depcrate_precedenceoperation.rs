// Generated macro for Operation (enum)
macro_rules! Depcrate_precedenceOperation {
() => {
// Module: crate::precedence
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " A single evaluation step."] pub enum Operation < P1 , P2 , P3 , O > { # [doc = " A prefix operation."] Prefix (P1 , O) , # [doc = " A postfix operation."] Postfix (O , P2) , # [doc = " A binary operation."] Binary (O , P3 , O) , }
};
}
