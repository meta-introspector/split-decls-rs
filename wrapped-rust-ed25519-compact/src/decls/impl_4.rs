macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [cfg (feature = "random")] impl Seed { # [doc = " Generates a random seed."] pub fn generate () -> Self { Seed :: default () } }
    };
}

impl_4!();