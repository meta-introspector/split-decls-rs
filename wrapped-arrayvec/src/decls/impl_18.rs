macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < const CAP : usize > AsRef < str > for ArrayString < CAP > { fn as_ref (& self) -> & str { self } }
    };
}

impl_18!();