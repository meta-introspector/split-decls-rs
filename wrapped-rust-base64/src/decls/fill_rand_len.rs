macro_rules! fill_rand_len {
    () => {
        fn fill_rand_len < R : rand :: Rng > (vec : & mut Vec < u8 > , rng : & mut R , len : usize) { for _ in 0 .. len { vec . push (rng . gen ()) ; } }
    };
}

fill_rand_len!();