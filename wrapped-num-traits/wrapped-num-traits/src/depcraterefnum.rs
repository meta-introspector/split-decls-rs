// Generated macro for RefNum (trait)
macro_rules! DepcrateRefNum {
() => {
// Module: crate
// Provides: {"RefNum"}
// Dependencies: {}
# [doc = " The trait for `Num` references which implement numeric operations, taking the"] # [doc = " second operand either by value or by reference."] # [doc = ""] # [doc = " This is automatically implemented for all types which implement the operators. It covers"] # [doc = " every type implementing the operations though, regardless of it being a reference or"] # [doc = " related to `Num`."] pub trait RefNum < Base > : NumOps < Base , Base > + for < 'r > NumOps < & 'r Base , Base > { }
};
}
