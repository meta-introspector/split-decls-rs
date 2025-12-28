macro_rules! deps {
    () => {
        Distribution!();
        Rng!();
        Choose!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a , T > Distribution < & 'a T > for Choose < 'a , T > { fn sample < R : crate :: Rng + ? Sized > (& self , rng : & mut R) -> & 'a T { let idx = self . range . sample (rng) ; debug_assert ! (idx < self . slice . len () , "Uniform::new(0, {}) somehow returned {}" , self . slice . len () , idx) ; unsafe { self . slice . get_unchecked (idx) } } }
    };
}

impl_119!();