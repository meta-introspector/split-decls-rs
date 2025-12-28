macro_rules! deps {
    () => {
        Chunks!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < I > Chunks < I > { # [doc = " Creates a new `Chunks` iterator"] pub (super) fn new (i : I , size : usize) -> Self { Chunks { i , size } } }
    };
}

impl_316!();