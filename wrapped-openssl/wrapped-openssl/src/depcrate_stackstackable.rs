// Generated macro for Stackable (trait)
macro_rules! Depcrate_stackStackable {
() => {
// Module: crate::stack
// Provides: {"Stackable"}
// Dependencies: {}
# [doc = " Trait implemented by types which can be placed in a stack."] # [doc = ""] # [doc = " It should not be implemented for any type outside of this crate."] pub trait Stackable : ForeignType { # [doc = " The C stack type for this element."] # [doc = ""] # [doc = " Generally called `stack_st_{ELEMENT_TYPE}`, normally hidden by the"] # [doc = " `STACK_OF(ELEMENT_TYPE)` macro in the OpenSSL API."] type StackType ; }
};
}
