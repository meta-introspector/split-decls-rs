macro_rules! deps {
    () => {
        Decoder!();
    };
}

macro_rules! SerdeDecoder {
    () => {
        deps!();
        pub (super) struct SerdeDecoder < 'a , DE : Decoder > { pub (super) de : & 'a mut DE , }
    };
}

SerdeDecoder!()