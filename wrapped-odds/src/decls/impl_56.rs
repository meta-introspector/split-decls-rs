macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T > From < Box < [T] > > for Box < RevSlice < T > > { fn from (slc : Box < [T] >) -> Self { unsafe { transmute (slc) } } }
    };
}

impl_56!()