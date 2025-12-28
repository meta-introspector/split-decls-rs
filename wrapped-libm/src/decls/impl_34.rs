macro_rules! deps {
    () => {
        DInt!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl DInt for i256 { type H = i128 ; fn lo (self) -> Self :: H { self . lo as i128 } fn hi (self) -> Self :: H { self . hi } }
    };
}

impl_34!();