macro_rules! deps {
    () => {
        HeaderSlice!();
        ThinArc!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < H , T > Deref for ThinArc < H , T > { type Target = HeaderSlice < H , [T] > ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { & (* thin_to_thick (self . ptr . as_ptr ())) . data } } }
    };
}

impl_154!();