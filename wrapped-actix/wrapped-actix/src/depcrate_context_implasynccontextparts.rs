// Generated macro for AsyncContextParts (trait)
macro_rules! Depcrate_context_implAsyncContextParts {
() => {
// Module: crate::context_impl
// Provides: {"AsyncContextParts"}
// Dependencies: {}
pub trait AsyncContextParts < A > : ActorContext + AsyncContext < A > where A : Actor < Context = Self > , { fn parts (& mut self) -> & mut ContextParts < A > ; }
};
}
