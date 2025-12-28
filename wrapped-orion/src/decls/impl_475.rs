macro_rules! deps {
    () => {
        PkeParameters!();
        DecapKey!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > PartialEq < & [u8] > for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn eq (& self , other : & & [u8]) -> bool { use subtle :: ConstantTimeEq ; (self . unprotected_as_bytes () . ct_eq (* other)) . into () } }
    };
}

impl_475!()