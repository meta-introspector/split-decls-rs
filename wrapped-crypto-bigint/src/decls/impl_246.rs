macro_rules! deps {
    () => {
        Odd!();
        Random!();
        Uint!();
        Limb!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        # [cfg (feature = "rand_core")] impl < const LIMBS : usize > Random for Odd < Uint < LIMBS > > { # [doc = " Generate a random `Odd<Uint<T>>`."] fn try_random < R : TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { let mut ret = Uint :: try_random (rng) ? ; ret . limbs [0] |= Limb :: ONE ; Ok (Odd (ret)) } }
    };
}

impl_246!();