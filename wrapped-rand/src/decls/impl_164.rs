macro_rules! deps {
    () => {
        Distribution!();
        Uniform!();
        Rng!();
        SampleUniform!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < X : SampleUniform > Distribution < X > for Uniform < X > { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> X { self . 0 . sample (rng) } }
    };
}

impl_164!();