macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl PartialEq for FixedBitSet { fn eq (& self , other : & Self) -> bool { self . length == other . length && self . as_simd_slice () . eq (other . as_simd_slice ()) } }
    };
}

impl_84!()