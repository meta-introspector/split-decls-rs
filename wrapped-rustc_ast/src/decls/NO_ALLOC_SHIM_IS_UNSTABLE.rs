macro_rules! NO_ALLOC_SHIM_IS_UNSTABLE {
    () => {
        pub const NO_ALLOC_SHIM_IS_UNSTABLE : & str = "__rust_no_alloc_shim_is_unstable_v2" ;
    };
}

NO_ALLOC_SHIM_IS_UNSTABLE!();