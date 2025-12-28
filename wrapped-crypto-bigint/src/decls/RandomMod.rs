macro_rules! deps {
    () => {
        NonZero!();
        Zero!();
    };
}

macro_rules! RandomMod {
    () => {
        deps!();
        # [doc = " Modular random number generation support."] # [cfg (feature = "rand_core")] pub trait RandomMod : Sized + Zero { # [doc = " Generate a random number which is less than a given `modulus`."] # [doc = ""] # [doc = " This uses rejection sampling."] # [doc = ""] # [doc = " As a result, it runs in variable time that depends in part on"] # [doc = " `modulus`. If the generator `rng` is cryptographically secure (for"] # [doc = " example, it implements `CryptoRng`), then this is guaranteed not to"] # [doc = " leak anything about the output value aside from it being less than"] # [doc = " `modulus`."] fn random_mod < R : RngCore + ? Sized > (rng : & mut R , modulus : & NonZero < Self >) -> Self { let Ok (out) = Self :: try_random_mod (rng , modulus) ; out } # [doc = " Generate a random number which is less than a given `modulus`."] # [doc = ""] # [doc = " This uses rejection sampling."] # [doc = ""] # [doc = " As a result, it runs in variable time that depends in part on"] # [doc = " `modulus`. If the generator `rng` is cryptographically secure (for"] # [doc = " example, it implements `CryptoRng`), then this is guaranteed not to"] # [doc = " leak anything about the output value aside from it being less than"] # [doc = " `modulus`."] fn try_random_mod < R : TryRngCore + ? Sized > (rng : & mut R , modulus : & NonZero < Self > ,) -> Result < Self , R :: Error > ; }
    };
}

RandomMod!()