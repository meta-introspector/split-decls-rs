// Generated macro for StackString (struct)
macro_rules! Depcrate_stackStackString {
() => {
// Module: crate::stack
// Provides: {"StackString"}
// Dependencies: {}
# [doc = " Fixed-size stack-allocated string"] # [derive (Copy , Clone)] pub struct StackString < const CAPACITY : usize > { len : Len , buffer : StrBuffer < CAPACITY > , }
};
}
