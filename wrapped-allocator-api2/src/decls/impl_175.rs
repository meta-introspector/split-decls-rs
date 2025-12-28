macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < T , A : Allocator > Drop for Vec < T , A > { # [inline (always)] fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . as_mut_ptr () , self . len)) } } }
    };
}

impl_175!();