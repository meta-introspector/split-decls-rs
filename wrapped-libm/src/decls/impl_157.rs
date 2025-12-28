macro_rules! deps {
    () => {
        DInt!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl DInt for u256 { type H = u128 ; fn lo (self) -> Self :: H { self . lo } fn hi (self) -> Self :: H { self . hi } }
    };
}

impl_157!();