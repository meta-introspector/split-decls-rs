macro_rules! deps {
    () => {
        ArrayGuard!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T , const N : usize > Drop for ArrayGuard < T , N > { fn drop (& mut self) { debug_assert ! (self . initialized <= N) ; let initialized_part = ptr :: slice_from_raw_parts_mut (self . dst , self . initialized) ; unsafe { ptr :: drop_in_place (initialized_part) ; } } }
    };
}

impl_40!()