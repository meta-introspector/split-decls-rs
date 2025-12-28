macro_rules! deps {
    () => {
        MappedErr!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < T , U > From < T > for MappedErr < T , U > { # [inline] fn from (t : T) -> MappedErr < T , U > { MappedErr :: It (t) } }
    };
}

impl_102!()