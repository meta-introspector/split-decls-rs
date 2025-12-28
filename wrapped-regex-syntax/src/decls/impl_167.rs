macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl From < u8 > for Literal { fn from (byte : u8) -> Literal { Literal :: exact (vec ! [byte]) } }
    };
}

impl_167!()