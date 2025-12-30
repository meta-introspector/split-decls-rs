// Generated macro for is_maybe_sized_bound (function)
macro_rules! Depcrate_deriving_coerce_pointeeis_maybe_sized_bound {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"is_maybe_sized_bound"}
// Dependencies: {}
fn is_maybe_sized_bound (bound : & GenericBound) -> bool { if let GenericBound :: Trait (trait_ref) = bound && let TraitBoundModifiers { polarity : ast :: BoundPolarity :: Maybe (_) , .. } = trait_ref . modifiers && is_sized_marker (& trait_ref . trait_ref . path) { true } else { false } }
};
}
