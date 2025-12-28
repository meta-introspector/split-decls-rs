macro_rules! deps {
    () => {
        ArrayLength!();
        IntrusiveArrayConsumer!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Drop for IntrusiveArrayConsumer < '_ , T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (self . position ..)) ; } } }
    };
}

impl_155!()