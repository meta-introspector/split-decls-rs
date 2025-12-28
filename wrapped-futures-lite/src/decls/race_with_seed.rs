macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! race_with_seed {
    () => {
        deps!();
        # [doc = " Races two streams, but with a user-provided seed for randomness."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, once, pending, StreamExt};"] # [doc = ""] # [doc = " // A fixed seed is used for reproducibility."] # [doc = " const SEED: u64 = 123;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(stream::race_with_seed(once(1), pending(), SEED).next().await, Some(1));"] # [doc = " assert_eq!(stream::race_with_seed(pending(), once(2), SEED).next().await, Some(2));"] # [doc = ""] # [doc = " // One of the two stream is randomly chosen as the winner."] # [doc = " let res = stream::race_with_seed(once(1), once(2), SEED).next().await;"] # [doc = " # })"] # [doc = " ```"] # [cfg (feature = "race")] pub fn race_with_seed < T , S1 , S2 > (stream1 : S1 , stream2 : S2 , seed : u64) -> Race < S1 , S2 > where S1 : Stream < Item = T > , S2 : Stream < Item = T > , { Race { stream1 , stream2 , rng : Rng :: with_seed (seed) , } }
    };
}

race_with_seed!()