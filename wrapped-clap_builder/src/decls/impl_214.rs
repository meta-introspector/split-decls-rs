macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for Str { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
    };
}

impl_214!()