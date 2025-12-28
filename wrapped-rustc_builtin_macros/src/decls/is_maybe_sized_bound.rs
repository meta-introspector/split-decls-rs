macro_rules! is_maybe_sized_bound {
    () => {
        fn is_maybe_sized_bound (bound : & GenericBound) -> bool { if let GenericBound :: Trait (trait_ref) = bound && let TraitBoundModifiers { polarity : ast :: BoundPolarity :: Maybe (_) , .. } = trait_ref . modifiers && is_sized_marker (& trait_ref . trait_ref . path) { true } else { false } }
    };
}

is_maybe_sized_bound!();