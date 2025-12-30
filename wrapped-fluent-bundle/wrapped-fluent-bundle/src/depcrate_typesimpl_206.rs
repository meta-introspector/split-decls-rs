// Generated macro for impl_206 (impl)
macro_rules! Depcrate_typesimpl_206 {
() => {
// Module: crate::types
// Provides: {"impl_206"}
// Dependencies: {}
impl < T : Any + PartialEq > AnyEq for T { fn equals (& self , other : & dyn Any) -> bool { other . downcast_ref :: < Self > () == Some (self) } fn as_any (& self) -> & dyn Any { self } }
};
}
