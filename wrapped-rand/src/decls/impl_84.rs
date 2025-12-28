macro_rules! deps {
    () => {
        StandardUniform!();
        Distribution!();
        Rng!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T , const N : usize > Distribution < [T ; N] > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [T ; N] { array :: from_fn (| _ | rng . random ()) } }
    };
}

impl_84!()