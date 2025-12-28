macro_rules! deps {
    () => {
        LineRange!();
        Offset!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl LineRange for Range < u32 > { fn shift_by (& self , offset : Offset) -> Self { offset . shifted_range (self) } }
    };
}

impl_18!();