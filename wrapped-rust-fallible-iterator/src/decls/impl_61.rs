macro_rules! deps {
    () => {
        MappedErr!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T , U > From < T > for MappedErr < T , U > { # [inline] fn from (t : T) -> MappedErr < T , U > { MappedErr :: It (t) } }
    };
}

impl_61!()