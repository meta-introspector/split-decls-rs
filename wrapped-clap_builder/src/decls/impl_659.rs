macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl PartialEq < str > for Id { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . as_str () , other) } }
    };
}

impl_659!();