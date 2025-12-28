macro_rules! deps {
    () => {
        EncapKey!();
        PkeParameters!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE : usize , Pke : PkeParameters > AsRef < [u8] > for EncapKey < K , ENCODED_SIZE , Pke > { fn as_ref (& self) -> & [u8] { self . bytes . as_ref () } }
    };
}

impl_468!()