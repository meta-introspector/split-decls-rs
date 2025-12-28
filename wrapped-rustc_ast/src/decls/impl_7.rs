macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl PartialEq < Symbol > for Path { # [inline] fn eq (& self , name : & Symbol) -> bool { if let [segment] = self . segments . as_ref () && segment == name { true } else { false } } }
    };
}

impl_7!();