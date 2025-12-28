macro_rules! deps {
    () => {
        PkeParameters!();
        DecapKey!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > Drop for DecapKey < K , ENCODED_SIZE_EK , ENCODED_SIZE_DK , Pke > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; self . s_hat . iter_mut () . zeroize () ; } }
    };
}

impl_472!();