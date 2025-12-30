// Generated macro for impl_363 (impl)
macro_rules! Depcrate_nameimpl_363 {
() => {
// Module: crate::name
// Provides: {"impl_363"}
// Dependencies: {}
impl < 'ns > TryFrom < ResolveResult < 'ns > > for Option < Namespace < 'ns > > { type Error = NamespaceError ; # [doc = " Try to convert this result to an optional namespace and returns"] # [doc = " [`NamespaceError::UnknownPrefix`] if this result represents unknown prefix"] fn try_from (result : ResolveResult < 'ns >) -> Result < Self , NamespaceError > { use ResolveResult :: * ; match result { Unbound => Ok (None) , Bound (ns) => Ok (Some (ns)) , Unknown (p) => Err (NamespaceError :: UnknownPrefix (p)) , } } }
};
}
