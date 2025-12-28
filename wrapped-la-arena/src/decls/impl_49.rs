macro_rules! deps {
    () => {
        Arena!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T > AsMut < [T] > for Arena < T > { fn as_mut (& mut self) -> & mut [T] { self . data . as_mut () } }
    };
}

impl_49!()