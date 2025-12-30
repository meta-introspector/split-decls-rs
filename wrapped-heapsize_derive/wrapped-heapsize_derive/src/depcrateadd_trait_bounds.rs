// Generated macro for add_trait_bounds (function)
macro_rules! Depcrateadd_trait_bounds {
() => {
// Module: crate
// Provides: {"add_trait_bounds"}
// Dependencies: {}
fn add_trait_bounds (mut generics : Generics) -> Generics { for param in & mut generics . params { if let GenericParam :: Type (ref mut type_param) = * param { type_param . bounds . push (parse_quote ! (heapsize :: HeapSize)) ; } } generics }
};
}
