macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl PartialEq < Bytes > for Vec < u8 > { fn eq (& self , other : & Bytes) -> bool { * other == * self } }
    };
}

impl_94!()