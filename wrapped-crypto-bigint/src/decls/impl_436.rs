macro_rules! deps {
    () => {
        Random!();
        Wrapping!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        # [cfg (feature = "rand_core")] impl < T : Random > Random for Wrapping < T > { fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { Ok (Wrapping (Random :: try_random (rng) ?)) } }
    };
}

impl_436!();