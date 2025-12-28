macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl PartialEq for Encoding { # [inline] fn eq (& self , other : & Encoding) -> bool { (self as * const Encoding) == (other as * const Encoding) } }
    };
}

impl_498!();