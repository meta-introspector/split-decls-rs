macro_rules! deps {
    () => {
        RingElementNTT!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl IndexMut < usize > for RingElementNTT { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { debug_assert ! (index <= 255) ; & mut self . coefficients [index] } }
    };
}

impl_437!()