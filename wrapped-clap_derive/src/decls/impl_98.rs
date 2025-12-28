macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < U , T : PartialEq < U > > PartialEq < U > for Sp < T > { fn eq (& self , other : & U) -> bool { self . val == * other } }
    };
}

impl_98!();