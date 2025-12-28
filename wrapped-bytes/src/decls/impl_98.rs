macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl PartialEq < Bytes > for String { fn eq (& self , other : & Bytes) -> bool { * other == * self } }
    };
}

impl_98!()