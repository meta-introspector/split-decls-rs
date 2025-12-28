macro_rules! iota {
    () => {
        fn iota (state : & mut [u64 ; 25] , round : usize) { debug_assert ! (round <= 24) ; state [0] ^= RC [round] ; }
    };
}

iota!();