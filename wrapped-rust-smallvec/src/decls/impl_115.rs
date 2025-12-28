macro_rules! deps {
    () => {
        DropGuard!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T > Drop for DropGuard < T > { # [inline] fn drop (& mut self) { unsafe { core :: ptr :: slice_from_raw_parts_mut (self . ptr , self . len) . drop_in_place () ; } } }
    };
}

impl_115!()