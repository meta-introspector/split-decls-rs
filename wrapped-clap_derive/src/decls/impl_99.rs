macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < T : AsRef < str > > AsRef < str > for Sp < T > { fn as_ref (& self) -> & str { self . val . as_ref () } }
    };
}

impl_99!();