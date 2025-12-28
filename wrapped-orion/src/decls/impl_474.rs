macro_rules! deps {
    () => {
        PkeParameters!();
        DecapKey!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > Eq for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { }
    };
}

impl_474!()