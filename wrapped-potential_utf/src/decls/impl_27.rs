macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl PartialEq < PotentialUtf8 > for str { fn eq (& self , other : & PotentialUtf8) -> bool { PotentialUtf8 :: from_str (self) . eq (other) } }
    };
}

impl_27!();