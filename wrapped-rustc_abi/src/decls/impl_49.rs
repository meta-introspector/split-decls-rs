macro_rules! deps {
    () => {
        Float!();
        Size!();
        HasDataLayout!();
        AbiAlign!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Float { pub fn size (self) -> Size { use Float :: * ; match self { F16 => Size :: from_bits (16) , F32 => Size :: from_bits (32) , F64 => Size :: from_bits (64) , F128 => Size :: from_bits (128) , } } pub fn align < C : HasDataLayout > (self , cx : & C) -> AbiAlign { use Float :: * ; let dl = cx . data_layout () ; match self { F16 => dl . f16_align , F32 => dl . f32_align , F64 => dl . f64_align , F128 => dl . f128_align , } } }
    };
}

impl_49!()