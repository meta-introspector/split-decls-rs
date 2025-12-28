macro_rules! deps {
    () => {
        RingElementNTT!();
        FieldElement!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl Index < usize > for RingElementNTT { type Output = FieldElement ; fn index (& self , index : usize) -> & Self :: Output { debug_assert ! (index <= 255) ; & self . coefficients [index] } }
    };
}

impl_436!()