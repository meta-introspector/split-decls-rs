macro_rules! deps {
    () => {
        RingElement!();
        FieldElement!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl Index < usize > for RingElement { type Output = FieldElement ; fn index (& self , index : usize) -> & Self :: Output { debug_assert ! (index <= 255) ; & self . coefficients [index] } }
    };
}

impl_429!();