macro_rules! o_of_q {
    () => {
        # [inline (always)] fn o_of_q (q : [u64 ; 2]) -> u128 { u128 :: from (q [0]) | (u128 :: from (q [1]) << 64) }
    };
}

o_of_q!()