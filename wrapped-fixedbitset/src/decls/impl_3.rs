macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { Self (self . 0 . not ()) } }
    };
}

impl_3!();