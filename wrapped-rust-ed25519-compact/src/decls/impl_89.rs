macro_rules! deps {
    () => {
        Noise!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "random")] impl Default for Noise { # [doc = " Generates random noise."] fn default () -> Self { let mut noise = [0u8 ; Noise :: BYTES] ; getrandom :: getrandom (& mut noise) . expect ("RNG failure") ; Noise (noise) } }
    };
}

impl_89!()