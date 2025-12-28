macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
    };
}

macro_rules! encode_slice_len {
    () => {
        deps!();
        # [doc = " Encodes the length of any slice, container, etc into the given encoder"] # [inline] pub (crate) fn encode_slice_len < E : Encoder > (encoder : & mut E , len : usize) -> Result < () , EncodeError > { (len as u64) . encode (encoder) }
    };
}

encode_slice_len!()