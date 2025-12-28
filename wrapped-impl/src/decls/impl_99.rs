macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl PartialEq < str > for IdentUnraw { fn eq (& self , other : & str) -> bool { self . 0 == other } }
    };
}

impl_99!();