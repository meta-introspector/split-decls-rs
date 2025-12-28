macro_rules! deps {
    () => {
        MixedBitSet!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > MixedBitSet < T > { pub fn domain_size (& self) -> usize { match self { MixedBitSet :: Small (set) => set . domain_size () , MixedBitSet :: Large (set) => set . domain_size () , } } }
    };
}

impl_40!();