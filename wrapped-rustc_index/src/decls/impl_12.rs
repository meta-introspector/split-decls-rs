macro_rules! deps {
    () => {
        DenseBitSet!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T > DenseBitSet < T > { # [doc = " Gets the domain size."] pub fn domain_size (& self) -> usize { self . domain_size } }
    };
}

impl_12!();