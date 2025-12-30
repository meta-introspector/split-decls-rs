// Generated macro for NumAssignRef (trait)
macro_rules! DepcrateNumAssignRef {
() => {
// Module: crate
// Provides: {"NumAssignRef"}
// Dependencies: {}
# [doc = " The trait for `NumAssign` types which also implement assignment operations"] # [doc = " taking the second operand by reference."] # [doc = ""] # [doc = " This is automatically implemented for types which implement the operators."] pub trait NumAssignRef : NumAssign + for < 'r > NumAssignOps < & 'r Self > { }
};
}
