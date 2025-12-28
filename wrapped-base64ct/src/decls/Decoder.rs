macro_rules! deps {
    () => {
        Line!();
        BlockBuffer!();
        LineReader!();
        Encoding!();
    };
}

macro_rules! Decoder {
    () => {
        deps!();
        # [doc = " Stateful Base64 decoder with support for buffered, incremental decoding."] # [doc = ""] # [doc = " The `E` type parameter can be any type which impls [`Encoding`] such as"] # [doc = " [`Base64`] or [`Base64Unpadded`]."] # [derive (Clone)] pub struct Decoder < 'i , E : Encoding > { # [doc = " Current line being processed."] line : Line < 'i > , # [doc = " Base64 input data reader."] line_reader : LineReader < 'i > , # [doc = " Length of the remaining data after Base64 decoding."] remaining_len : usize , # [doc = " Block buffer used for non-block-aligned data."] block_buffer : BlockBuffer , # [doc = " Phantom parameter for the Base64 encoding in use."] encoding : PhantomData < E > , }
    };
}

Decoder!();