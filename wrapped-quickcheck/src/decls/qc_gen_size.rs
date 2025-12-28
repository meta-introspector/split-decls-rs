macro_rules! qc_gen_size {
    () => {
        fn qc_gen_size () -> usize { let default = 100 ; match env :: var ("QUICKCHECK_GENERATOR_SIZE") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
    };
}

qc_gen_size!();