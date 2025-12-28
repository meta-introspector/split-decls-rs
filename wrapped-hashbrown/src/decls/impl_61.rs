macro_rules! deps {
    () => {
        RawTable!();
        RawTableClone!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : Clone , A : Allocator + Clone > RawTableClone for RawTable < T , A > { default_fn ! { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn clone_from_spec (& mut self , source : & Self) { self . clone_from_impl (source) ; } } }
    };
}

impl_61!()