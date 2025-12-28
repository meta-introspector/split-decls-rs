macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl PartialEq < Name > for Symbol { fn eq (& self , name : & Name) -> bool { * self == name . symbol } }
    };
}

impl_145!();