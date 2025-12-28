macro_rules! permutation_p {
    () => {
        # [allow (clippy :: too_many_arguments)] fn permutation_p (v0 : & mut u64 , v1 : & mut u64 , v2 : & mut u64 , v3 : & mut u64 , v4 : & mut u64 , v5 : & mut u64 , v6 : & mut u64 , v7 : & mut u64 , v8 : & mut u64 , v9 : & mut u64 , v10 : & mut u64 , v11 : & mut u64 , v12 : & mut u64 , v13 : & mut u64 , v14 : & mut u64 , v15 : & mut u64 ,) { g (v0 , v4 , v8 , v12) ; g (v1 , v5 , v9 , v13) ; g (v2 , v6 , v10 , v14) ; g (v3 , v7 , v11 , v15) ; g (v0 , v5 , v10 , v15) ; g (v1 , v6 , v11 , v12) ; g (v2 , v7 , v8 , v13) ; g (v3 , v4 , v9 , v14) ; }
    };
}

permutation_p!();