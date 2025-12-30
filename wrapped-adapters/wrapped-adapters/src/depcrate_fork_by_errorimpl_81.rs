// Generated macro for impl_81 (impl)
macro_rules! Depcrate_fork_by_errorimpl_81 {
() => {
// Module: crate::fork::by_error
// Provides: {"impl_81"}
// Dependencies: {}
impl < P , F > MultiForkByErrorProvider < P , F > { # [doc = " Create a new provider that forks between the vector of children."] # [doc = ""] # [doc = " The `predicate` argument should be an instance of a struct implementing"] # [doc = " [`ForkByErrorPredicate`]."] pub fn new_with_predicate (providers : Vec < P > , predicate : F) -> Self { Self { providers , predicate , } } # [doc = " Returns a slice of the inner providers."] pub fn inner (& self) -> & [P] { & self . providers } # [doc = " Exposes a mutable vector of providers to a closure so it can be mutated."] pub fn with_inner_mut (& mut self , f : impl FnOnce (& mut Vec < P >)) { f (& mut self . providers) } # [doc = " Returns ownership of the inner providers to the caller."] pub fn into_inner (self) -> Vec < P > { self . providers } # [doc = " Adds an additional child provider."] pub fn push (& mut self , provider : P) { self . providers . push (provider) ; } }
};
}
