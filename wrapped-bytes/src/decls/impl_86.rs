macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl PartialEq < Bytes > for [u8] { fn eq (& self , other : & Bytes) -> bool { * other == * self } }
    };
}

impl_86!()