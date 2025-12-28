macro_rules! deps {
    () => {
        ArrayLength!();
        ArrayConsumer!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Drop for ArrayConsumer < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (self . position ..)) ; } } }
    };
}

impl_152!();