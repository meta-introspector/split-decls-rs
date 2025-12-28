macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < T , A : Allocator > Drop for RawDrain < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . iter . drop_elements () ; self . table . clear_no_drop () ; self . orig_table . as_ptr () . copy_from_nonoverlapping (& self . table , 1) ; } } }
    };
}

impl_101!()