macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T > IndexMut < usize > for RingBuffer < T > { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . data [index . checked_sub (self . offset) . unwrap ()] } }
    };
}

impl_77!();