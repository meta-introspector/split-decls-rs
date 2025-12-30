// Generated macro for impl_73 (impl)
macro_rules! Depcrate_fork_by_errorimpl_73 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_73"}
// Dependencies: {}
impl < P0 , P1 , F > ForkByErrorProvider < P0 , P1 , F > { # [doc = " Create a new provider that forks between the two children."] # [doc = ""] # [doc = " The `predicate` argument should be an instance of a struct implementing"] # [doc = " [`ForkByErrorPredicate`]."] pub fn new_with_predicate (p0 : P0 , p1 : P1 , predicate : F) -> Self { Self (p0 , p1 , predicate) } # [doc = " Returns references to the inner providers."] pub fn inner (& self) -> (& P0 , & P1) { (& self . 0 , & self . 1) } # [doc = " Returns mutable references to the inner providers."] pub fn inner_mut (& mut self) -> (& mut P0 , & mut P1) { (& mut self . 0 , & mut self . 1) } # [doc = " Returns ownership of the inner providers to the caller."] pub fn into_inner (self) -> (P0 , P1) { (self . 0 , self . 1) } }
};
}
