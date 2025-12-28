macro_rules! deps {
    () => {
        RingElement!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl IndexMut < usize > for RingElement { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { debug_assert ! (index <= 255) ; & mut self . coefficients [index] } }
    };
}

impl_430!();