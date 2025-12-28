macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl PartialEq < String > for Bytes { fn eq (& self , other : & String) -> bool { * self == other [..] } }
    };
}

impl_96!()