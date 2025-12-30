// Generated macro for NamespaceBindingsOfLevelIter (struct)
macro_rules! Depcrate_nameNamespaceBindingsOfLevelIter {
() => {
// Module: crate::name
// Provides: {"NamespaceBindingsOfLevelIter"}
// Dependencies: {}
# [doc = " Iterator on the declared namespace bindings on specified level. Returns pairs of the _(prefix, namespace)_."] # [doc = ""] # [doc = " See [`NamespaceResolver::bindings_of`] for documentation."] # [derive (Debug , Clone)] pub struct NamespaceBindingsOfLevelIter < 'a > { resolver : & 'a NamespaceResolver , cursor : usize , level : u16 , }
};
}
