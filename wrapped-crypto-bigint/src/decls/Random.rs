macro_rules! Random {
    () => {
        # [doc = " Random number generation support."] # [cfg (feature = "rand_core")] pub trait Random : Sized { # [doc = " Generate a random value."] # [doc = ""] # [doc = " If `rng` is a CSRNG, the generation is cryptographically secure as well."] fn random < R : RngCore + ? Sized > (rng : & mut R) -> Self { let Ok (out) = Self :: try_random (rng) ; out } # [doc = " Generate a random value."] # [doc = ""] # [doc = " If `rng` is a CSRNG, the generation is cryptographically secure as well."] fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > ; }
    };
}

Random!()