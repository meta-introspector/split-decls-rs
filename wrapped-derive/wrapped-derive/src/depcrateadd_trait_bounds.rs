// Generated macro for add_trait_bounds (function)
macro_rules! Depcrateadd_trait_bounds {
() => {
// Module: crate
// Provides: {"add_trait_bounds"}
// Dependencies: {}
fn add_trait_bounds (mut generics : Generics , lifetime : LifetimeParam) -> Generics { for param in generics . params . iter_mut () { if let GenericParam :: Type (type_param) = param { type_param . bounds . push (parse_quote ! (arbitrary :: Arbitrary <# lifetime >)) ; } } generics }
};
}
