macro_rules! deps {
    () => {
        Regex!();
        Pool!();
        CachePoolFn!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl Clone for Regex { fn clone (& self) -> Regex { let imp = Arc :: clone (& self . imp) ; let pool = { let strat = Arc :: clone (& imp . strat) ; let create : CachePoolFn = Box :: new (move | | strat . create_cache ()) ; Pool :: new (create) } ; Regex { imp , pool } } }
    };
}

impl_339!();