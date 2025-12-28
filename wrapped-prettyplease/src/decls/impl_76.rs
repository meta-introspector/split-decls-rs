macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > Index < usize > for RingBuffer < T > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self . data [index . checked_sub (self . offset) . unwrap ()] } }
    };
}

impl_76!()