macro_rules! keccakf {
    () => {
        fn keccakf < const ROUNDS : usize > (state : & mut [u64 ; 25]) { for round in 0 .. ROUNDS { let mut buf = [0u64 ; 5] ; theta (state , & mut buf) ; rho_and_pi (state , & mut buf) ; chi (state , & mut buf) ; iota (state , round) ; } }
    };
}

keccakf!();