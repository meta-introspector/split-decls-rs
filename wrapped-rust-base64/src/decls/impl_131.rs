macro_rules! deps {
    () => {
        DecodeEstimate!();
        NaiveEstimate!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl DecodeEstimate for NaiveEstimate { fn decoded_len_estimate (& self) -> usize { ((self . complete_chunk_len / 4) + ((self . rem > 0) as usize)) * 3 } }
    };
}

impl_131!();