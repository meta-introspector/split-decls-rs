macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl PartialEq for Encoding { # [inline] fn eq (& self , other : & Encoding) -> bool { (self as * const Encoding) == (other as * const Encoding) } }
    };
}

impl_113!()