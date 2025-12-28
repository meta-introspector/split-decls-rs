macro_rules! CoinFlipper {
    () => {
        pub (crate) struct CoinFlipper < R : RngCore > { pub rng : R , chunk : u32 , chunk_remaining : u32 , }
    };
}

CoinFlipper!()