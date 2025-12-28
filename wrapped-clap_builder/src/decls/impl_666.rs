macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl PartialEq < Id > for String { # [inline] fn eq (& self , other : & Id) -> bool { PartialEq :: eq (other , self) } }
    };
}

impl_666!()