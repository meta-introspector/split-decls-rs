// Generated macro for impl_37 (impl)
macro_rules! Depcrate_expandimpl_37 {
() => {
// Module: crate::expand
// Provides: {"impl_37"}
// Dependencies: {}
impl Context < '_ > { fn lifetimes < 'a > (& 'a self , used : & 'a [Lifetime]) -> impl Iterator < Item = & 'a LifetimeParam > { let generics = match self { Context :: Trait { generics , .. } => generics , Context :: Impl { impl_generics , .. } => impl_generics , } ; generics . params . iter () . filter_map (move | param | { if let GenericParam :: Lifetime (param) = param { if used . contains (& param . lifetime) { return Some (param) ; } } None }) } }
};
}
