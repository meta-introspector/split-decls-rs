macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Ord for FixedBitSet { fn cmp (& self , other : & Self) -> Ordering { self . length . cmp (& other . length) . then_with (| | self . as_simd_slice () . cmp (other . as_simd_slice ())) } }
    };
}

impl_17!()