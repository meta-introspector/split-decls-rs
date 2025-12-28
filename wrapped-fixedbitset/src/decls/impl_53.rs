macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { ! v128_any_true (v128_xor (self . 0 , other . 0)) } }
    };
}

impl_53!();