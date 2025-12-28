macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (feature = "random")] impl Default for Seed { # [doc = " Generates a random seed."] fn default () -> Self { let mut seed = [0u8 ; Seed :: BYTES] ; getrandom :: getrandom (& mut seed) . expect ("RNG failure") ; Seed (seed) } }
    };
}

impl_3!();