macro_rules! deps {
    () => {
        Xoshiro256PlusPlus!();
    };
}

macro_rules! Rng {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] type Rng = super :: xoshiro256plusplus :: Xoshiro256PlusPlus ;
    };
}

Rng!()