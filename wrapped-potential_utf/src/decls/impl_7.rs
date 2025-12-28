macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl PartialEq < char > for PotentialCodePoint { fn eq (& self , other : & char) -> bool { self . eq (& Self :: from_char (* other)) } }
    };
}

impl_7!()