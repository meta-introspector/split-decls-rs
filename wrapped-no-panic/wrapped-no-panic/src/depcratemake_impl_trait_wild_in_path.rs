// Generated macro for make_impl_trait_wild_in_path (function)
macro_rules! Depcratemake_impl_trait_wild_in_path {
() => {
// Module: crate
// Provides: {"make_impl_trait_wild_in_path"}
// Dependencies: {}
fn make_impl_trait_wild_in_path (path : & mut Path) { for segment in & mut path . segments { if let PathArguments :: AngleBracketed (bracketed) = & mut segment . arguments { for arg in & mut bracketed . args { if let GenericArgument :: Type (arg) = arg { make_impl_trait_wild (arg) ; } } } } }
};
}
