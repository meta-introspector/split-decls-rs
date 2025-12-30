// Generated macro for InequalityKind (enum)
macro_rules! Depcrate_ir_pccInequalityKind {
() => {
// Module: crate::ir::pcc
// Provides: {"InequalityKind"}
// Dependencies: {}
# [doc = " The two kinds of inequalities: \"strict\" (`<`, `>`) and \"loose\""] # [doc = " (`<=`, `>=`), the latter of which admit equality."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum InequalityKind { # [doc = " Strict inequality: {less,greater}-than."] Strict , # [doc = " Loose inequality: {less,greater}-than-or-equal."] Loose , }
};
}
