macro_rules! deps {
    () => {
        Decoder!();
    };
}

macro_rules! OwnedSerdeDecoder {
    () => {
        deps!();
        # [doc = " Serde decoder encapsulating an owned reader."] pub struct OwnedSerdeDecoder < DE : Decoder > { pub (super) de : DE , }
    };
}

OwnedSerdeDecoder!()