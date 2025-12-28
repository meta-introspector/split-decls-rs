macro_rules! deps {
    () => {
        Noise!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "random")] impl Noise { # [doc = " Generates random noise."] pub fn generate () -> Self { Noise :: default () } }
    };
}

impl_90!();