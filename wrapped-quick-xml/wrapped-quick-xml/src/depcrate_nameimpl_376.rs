// Generated macro for impl_376 (impl)
macro_rules! Depcrate_nameimpl_376 {
() => {
// Module: crate::name
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'a > Iterator for NamespaceBindingsOfLevelIter < 'a > { type Item = (PrefixDeclaration < 'a > , Namespace < 'a >) ; fn next (& mut self) -> Option < (PrefixDeclaration < 'a > , Namespace < 'a >) > { while let Some (binding) = self . resolver . bindings . get (self . cursor) { self . cursor += 1 ; if binding . level < self . level { continue ; } if binding . level > self . level { break ; } if let ResolveResult :: Bound (namespace) = binding . namespace (& self . resolver . buffer) { let prefix = match binding . prefix (& self . resolver . buffer) { Some (Prefix (prefix)) => PrefixDeclaration :: Named (prefix) , None => PrefixDeclaration :: Default , } ; return Some ((prefix , namespace)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . resolver . bindings . len () - self . cursor)) } }
};
}
