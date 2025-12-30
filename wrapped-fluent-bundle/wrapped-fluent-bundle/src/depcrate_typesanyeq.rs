// Generated macro for AnyEq (trait)
macro_rules! Depcrate_typesAnyEq {
() => {
// Module: crate::types
// Provides: {"AnyEq"}
// Dependencies: {}
pub trait AnyEq : Any + 'static { fn equals (& self , other : & dyn Any) -> bool ; fn as_any (& self) -> & dyn Any ; }
};
}
