macro_rules! deps {
    () => {
        Serializer!();
        RawValue!();
        Result!();
        Error!();
    };
}

macro_rules! impl_616 {
    () => {
        deps!();
        impl Serialize for RawValue { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut s = tri ! (serializer . serialize_struct (TOKEN , 1)) ; tri ! (s . serialize_field (TOKEN , & self . json)) ; s . end () } }
    };
}

impl_616!()