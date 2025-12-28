macro_rules! deps {
    () => {
        DecapsulationKey!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl PartialEq < & [u8] > for DecapsulationKey { fn eq (& self , other : & & [u8]) -> bool { self . value == * other } }
    };
}

impl_528!()