macro_rules! deps {
    () => {
        GeneralPurposeEstimate!();
        DecodeEstimate!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl DecodeEstimate for GeneralPurposeEstimate { fn decoded_len_estimate (& self) -> usize { self . conservative_decoded_len } }
    };
}

impl_93!();