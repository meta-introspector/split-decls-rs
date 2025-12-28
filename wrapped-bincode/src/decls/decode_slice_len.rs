macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
    };
}

macro_rules! decode_slice_len {
    () => {
        deps!();
        # [doc = " Decodes the length of any slice, container, etc from the decoder"] # [inline] pub (crate) fn decode_slice_len < D : Decoder > (decoder : & mut D) -> Result < usize , DecodeError > { let v = u64 :: decode (decoder) ? ; v . try_into () . map_err (| _ | DecodeError :: OutsideUsizeRange (v)) }
    };
}

decode_slice_len!()