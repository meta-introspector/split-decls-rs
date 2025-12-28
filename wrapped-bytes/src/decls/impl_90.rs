macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl PartialEq < Bytes > for str { fn eq (& self , other : & Bytes) -> bool { * other == * self } }
    };
}

impl_90!();