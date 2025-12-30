// Generated macro for NumRef (trait)
macro_rules! DepcrateNumRef {
() => {
// Module: crate
// Provides: {"NumRef"}
// Dependencies: {}
# [doc = " The trait for `Num` types which also implement numeric operations taking"] # [doc = " the second operand by reference."] # [doc = ""] # [doc = " This is automatically implemented for types which implement the operators."] pub trait NumRef : Num + for < 'r > NumOps < & 'r Self > { }
};
}
