macro_rules! deps {
    () => {
        Encoder!();
        Writer!();
        Encode!();
        Config!();
    };
}

macro_rules! EncoderImpl {
    () => {
        deps!();
        # [doc = " An Encoder that writes bytes into a given writer `W`."] # [doc = ""] # [doc = " This struct should rarely be used."] # [doc = " In most cases, prefer any of the `encode` functions."] # [doc = ""] # [doc = " The ByteOrder that is chosen will impact the endianness that"] # [doc = " is used to write integers to the writer."] # [doc = ""] # [doc = " ```"] # [doc = " # use bincode::enc::{write::SliceWriter, EncoderImpl, Encode};"] # [doc = " let slice: &mut [u8] = &mut [0, 0, 0, 0];"] # [doc = " let config = bincode::config::legacy().with_big_endian();"] # [doc = ""] # [doc = " let mut encoder = EncoderImpl::new(SliceWriter::new(slice), config);"] # [doc = " // this u32 can be any Encodable"] # [doc = " 5u32.encode(&mut encoder).unwrap();"] # [doc = " assert_eq!(encoder.into_writer().bytes_written(), 4);"] # [doc = " assert_eq!(slice, [0, 0, 0, 5]);"] # [doc = " ```"] pub struct EncoderImpl < W : Writer , C : Config > { writer : W , config : C , }
    };
}

EncoderImpl!()