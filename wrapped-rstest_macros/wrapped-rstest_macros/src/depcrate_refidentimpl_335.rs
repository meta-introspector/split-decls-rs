// Generated macro for impl_335 (impl)
macro_rules! Depcrate_refidentimpl_335 {
() => {
// Module: crate::refident
// Provides: {"impl_335"}
// Dependencies: {}
impl MaybeType for FnArg { fn maybe_type (& self) -> Option < & Type > { match self { FnArg :: Typed (PatType { ty , .. }) => Some (ty . as_ref ()) , _ => None , } } }
};
}
