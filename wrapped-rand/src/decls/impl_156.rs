macro_rules! deps {
    () => {
        Rng!();
        StandardUniform!();
        Distribution!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T > Distribution < Wrapping < T > > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Wrapping < T > { Wrapping (rng . random ()) } }
    };
}

impl_156!()