// Generated macro for AnyClone (trait)
macro_rules! Depcrate_extensionsAnyClone {
() => {
// Module: crate::extensions
// Provides: {"AnyClone"}
// Dependencies: {}
trait AnyClone : Any { fn clone_box (& self) -> Box < dyn AnyClone + Send + Sync > ; fn as_any (& self) -> & dyn Any ; fn as_any_mut (& mut self) -> & mut dyn Any ; fn into_any (self : Box < Self >) -> Box < dyn Any > ; fn type_name (& self) -> & 'static str ; }
};
}
