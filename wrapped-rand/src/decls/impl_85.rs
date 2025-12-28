macro_rules! deps {
    () => {
        Distribution!();
        StandardUniform!();
        Rng!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T > Distribution < Wrapping < T > > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Wrapping < T > { Wrapping (rng . random ()) } }
    };
}

impl_85!()