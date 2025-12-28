macro_rules! deps {
    () => {
        GenericMachine!();
        Machine!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl Machine for GenericMachine { type u32x4 = u32x4_generic ; type u64x2 = u64x2_generic ; type u128x1 = u128x1_generic ; type u32x4x2 = u32x4x2_generic ; type u64x2x2 = u64x2x2_generic ; type u64x4 = u64x4_generic ; type u128x2 = u128x2_generic ; type u32x4x4 = u32x4x4_generic ; type u64x2x4 = u64x2x4_generic ; type u128x4 = u128x4_generic ; # [inline (always)] unsafe fn instance () -> Self { Self } }
    };
}

impl_301!();