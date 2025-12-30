// Generated macro for compute_impl_bounds (function)
macro_rules! Depcrate_codegen_outer_from_implcompute_impl_bounds {
() => {
// Module: crate::codegen::outer_from_impl
// Provides: {"compute_impl_bounds"}
// Dependencies: {}
fn compute_impl_bounds (bound : Path , mut generics : Generics , applies_to : & IdentSet) -> Generics { if generics . params . is_empty () { return generics ; } let added_bound = TypeParamBound :: Trait (TraitBound { paren_token : None , modifier : TraitBoundModifier :: None , lifetimes : None , path : bound , }) ; for param in generics . params . iter_mut () { if let GenericParam :: Type (ref mut typ) = * param { if applies_to . contains (& typ . ident) { typ . bounds . push (added_bound . clone ()) ; } } } generics }
};
}
