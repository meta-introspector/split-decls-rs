macro_rules! deps {
    () => {
        EncapsulationKey!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl PartialEq < & [u8] > for EncapsulationKey { fn eq (& self , other : & & [u8]) -> bool { self . value == * other } }
    };
}

impl_531!();