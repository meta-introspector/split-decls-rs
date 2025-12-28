macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { Self (v128_xor (self . 0 , Self :: ALL . 0)) } }
    };
}

impl_46!()