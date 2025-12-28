macro_rules! deps {
    () => {
        Rng!();
        Distribution!();
        StandardUniform!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T , const N : usize > Distribution < [T ; N] > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [T ; N] { array :: from_fn (| _ | rng . random ()) } }
    };
}

impl_155!();