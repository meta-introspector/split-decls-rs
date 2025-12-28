macro_rules! deps {
    () => {
        RawParDrain!();
        RawParIter!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T , A : Allocator > RawParDrain < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (super) unsafe fn par_iter (& self) -> RawParIter < T > { self . table . as_ref () . par_iter () } }
    };
}

impl_172!();