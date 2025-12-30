// Generated macro for impl_372 (impl)
macro_rules! Depcrate_nameimpl_372 {
() => {
// Module: crate::name
// Provides: {"impl_372"}
// Dependencies: {}
impl < 'a > Iterator for NamespaceBindingsIter < 'a > { type Item = (PrefixDeclaration < 'a > , Namespace < 'a >) ; fn next (& mut self) -> Option < (PrefixDeclaration < 'a > , Namespace < 'a >) > { while let Some (binding) = self . resolver . bindings . get (self . cursor) { self . cursor += 1 ; let prefix = binding . prefix (& self . resolver . buffer) ; if self . resolver . bindings [self . cursor ..] . iter () . any (| ne | prefix == ne . prefix (& self . resolver . buffer)) { continue ; } if let ResolveResult :: Bound (namespace) = binding . namespace (& self . resolver . buffer) { let prefix = match prefix { Some (Prefix (prefix)) => PrefixDeclaration :: Named (prefix) , None => PrefixDeclaration :: Default , } ; return Some ((prefix , namespace)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . resolver . bindings . len () - self . cursor)) } }
};
}
