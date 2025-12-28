macro_rules! test_atomic_load_store {
    () => {
        macro_rules ! test_atomic_load_store { ($ ty : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: arithmetic_side_effects , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks ,)] mod [< test_atomic_ $ ty >] { __test_atomic ! (load_store , $ ty) ; } } } ; }
    };
}

test_atomic_load_store!()