// Generated macro for impl_347 (impl)
macro_rules! Depcrate_refidentimpl_347 {
() => {
// Module: crate::refident
// Provides: {"impl_347"}
// Dependencies: {}
impl MaybePat for FnArg { fn maybe_pat (& self) -> Option < & syn :: Pat > { match self { FnArg :: Typed (PatType { pat , .. }) => Some (pat . as_ref ()) , _ => None , } } }
};
}
