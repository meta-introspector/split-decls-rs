macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_661 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for Id { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
    };
}

impl_661!();