macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl PartialEq for ConstChoice { fn eq (& self , other : & Self) -> bool { self . 0 == other . 0 } }
    };
}

impl_58!();