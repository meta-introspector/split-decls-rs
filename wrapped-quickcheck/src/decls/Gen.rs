macro_rules! deps {
    () => {
        QuickCheck!();
    };
}

macro_rules! Gen {
    () => {
        deps!();
        # [doc = " `Gen` represents a PRNG."] # [doc = ""] # [doc = " It is the source of randomness from which QuickCheck will generate values."] # [doc = " An instance of `Gen` is passed to every invocation of"] # [doc = " `Arbitrary::arbitrary`, which permits callers to use lower level RNG"] # [doc = " routines to generate values."] # [doc = ""] # [doc = " It is unspecified whether this is a secure RNG or not. Therefore, callers"] # [doc = " should assume it is insecure."] pub struct Gen { rng : rand :: rngs :: SmallRng , size : usize , }
    };
}

Gen!()