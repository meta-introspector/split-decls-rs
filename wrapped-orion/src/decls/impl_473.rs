macro_rules! deps {
    () => {
        PkeParameters!();
        DecapKey!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > PartialEq < DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > > for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn eq (& self , other : & DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke >) -> bool { use subtle :: ConstantTimeEq ; (self . unprotected_as_bytes () . ct_eq (other . unprotected_as_bytes ())) . into () } }
    };
}

impl_473!();