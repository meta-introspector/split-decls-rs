macro_rules! deps {
    () => {
        PkeParameters!();
        EncapKey!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE : usize , Pke : PkeParameters > PartialEq < & [u8] > for EncapKey < K , ENCODED_SIZE , Pke > { fn eq (& self , other : & & [u8]) -> bool { self . bytes == * other } }
    };
}

impl_469!();