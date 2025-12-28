macro_rules! build_arbitrary_lifetime {
    () => {
        fn build_arbitrary_lifetime (generics : Generics) -> (LifetimeParam , LifetimeParam) { let lifetime_without_bounds = LifetimeParam :: new (Lifetime :: new (ARBITRARY_LIFETIME_NAME , Span :: call_site ())) ; let mut lifetime_with_bounds = lifetime_without_bounds . clone () ; for param in generics . params . iter () { if let GenericParam :: Lifetime (lifetime_def) = param { lifetime_with_bounds . bounds . push (lifetime_def . lifetime . clone ()) ; } } (lifetime_without_bounds , lifetime_with_bounds) }
    };
}

build_arbitrary_lifetime!()