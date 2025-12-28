macro_rules! add_trait_bounds {
    () => {
        fn add_trait_bounds (mut generics : Generics , lifetime : LifetimeParam) -> Generics { for param in generics . params . iter_mut () { if let GenericParam :: Type (type_param) = param { type_param . bounds . push (parse_quote ! (arbitrary :: Arbitrary <# lifetime >)) ; } } generics }
    };
}

add_trait_bounds!()