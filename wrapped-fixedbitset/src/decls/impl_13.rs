macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { unsafe { Self (_mm_xor_si128 (self . 0 , Self :: ALL . 0)) } } }
    };
}

impl_13!()