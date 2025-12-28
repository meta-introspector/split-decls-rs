macro_rules! DecodeMetadata {
    () => {
        # [doc = " Metadata about the result of a decode operation"] # [derive (PartialEq , Eq , Debug)] pub struct DecodeMetadata { # [doc = " Number of decoded bytes output"] pub (crate) decoded_len : usize , # [doc = " Offset of the first padding byte in the input, if any"] pub (crate) padding_offset : Option < usize > , }
    };
}

DecodeMetadata!()