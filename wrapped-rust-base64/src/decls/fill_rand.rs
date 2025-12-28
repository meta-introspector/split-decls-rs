macro_rules! fill_rand {
    () => {
        fn fill_rand < R : rand :: Rng , D : distributions :: Distribution < usize > > (vec : & mut Vec < u8 > , rng : & mut R , length_distribution : & D ,) -> usize { let len = length_distribution . sample (rng) ; for _ in 0 .. len { vec . push (rng . gen ()) ; } len }
    };
}

fill_rand!();