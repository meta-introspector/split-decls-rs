macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl PartialEq < str > for PotentialUtf8 { fn eq (& self , other : & str) -> bool { self . eq (Self :: from_str (other)) } }
    };
}

impl_25!()