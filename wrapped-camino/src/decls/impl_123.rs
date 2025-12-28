macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl PartialOrd for Utf8Path { # [inline] fn partial_cmp (& self , other : & Utf8Path) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_123!()