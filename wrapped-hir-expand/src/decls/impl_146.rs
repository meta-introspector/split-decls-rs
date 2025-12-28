macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl PartialEq < Name > for & Symbol { fn eq (& self , name : & Name) -> bool { * * self == name . symbol } }
    };
}

impl_146!()