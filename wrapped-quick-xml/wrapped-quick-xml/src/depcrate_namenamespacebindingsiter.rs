// Generated macro for NamespaceBindingsIter (struct)
macro_rules! Depcrate_nameNamespaceBindingsIter {
() => {
// Module: crate::name
// Provides: {"NamespaceBindingsIter"}
// Dependencies: {}
# [doc = " Iterator on the current declared namespace bindings. Returns pairs of the _(prefix, namespace)_."] # [doc = ""] # [doc = " See [`NamespaceResolver::bindings`] for documentation."] # [derive (Debug , Clone)] pub struct NamespaceBindingsIter < 'a > { resolver : & 'a NamespaceResolver , cursor : usize , }
};
}
