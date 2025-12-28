macro_rules! test_atomic {
    () => {
        macro_rules ! test_atomic { ($ ty : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: arithmetic_side_effects , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks ,)] mod [< test_atomic_ $ ty >] { __test_atomic ! (load_store , $ ty) ; # [cfg (not (all (target_arch = "csky" , atomic_maybe_uninit_no_ldex_stex)))] __test_atomic ! (swap , $ ty) ; # [cfg (not (all (target_arch = "csky" , atomic_maybe_uninit_no_ldex_stex)))] # [cfg (not (all (target_arch = "x86" , atomic_maybe_uninit_no_cmpxchg)))] __test_atomic ! (cas , $ ty) ; } } } ; }
    };
}

test_atomic!()