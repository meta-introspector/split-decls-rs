macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl PartialEq < & Symbol > for Name { fn eq (& self , & sym : & & Symbol) -> bool { self . symbol == * sym } }
    };
}

impl_144!()