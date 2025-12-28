macro_rules! deps {
    () => {
        RawIntoParIter!();
        RawParIter!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T , A : Allocator > RawIntoParIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (super) unsafe fn par_iter (& self) -> RawParIter < T > { self . table . par_iter () } }
    };
}

impl_168!()