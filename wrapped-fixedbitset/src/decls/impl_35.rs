macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { unsafe { Self (_mm256_xor_si256 (self . 0 , Self :: ALL . 0)) } } }
    };
}

impl_35!()