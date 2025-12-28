macro_rules! deps {
    () => {
        Decoder!();
        Config!();
        Decode!();
    };
}

macro_rules! DecoderImpl {
    () => {
        deps!();
        # [doc = " A Decoder that reads bytes from a given reader `R`."] # [doc = ""] # [doc = " This struct should rarely be used."] # [doc = " In most cases, prefer any of the `decode` functions."] # [doc = ""] # [doc = " The ByteOrder that is chosen will impact the endianness that"] # [doc = " is used to read integers out of the reader."] # [doc = ""] # [doc = " ```"] # [doc = " # let slice: &[u8] = &[0, 0, 0, 0];"] # [doc = " # let some_reader = bincode::de::read::SliceReader::new(slice);"] # [doc = " use bincode::de::{DecoderImpl, Decode};"] # [doc = " let mut context = ();"] # [doc = " let mut decoder = DecoderImpl::new(some_reader, bincode::config::standard(), &mut context);"] # [doc = " // this u32 can be any Decode"] # [doc = " let value = u32::decode(&mut decoder).unwrap();"] # [doc = " ```"] pub struct DecoderImpl < R , C : Config , Context > { reader : R , config : C , bytes_read : usize , context : Context , }
    };
}

DecoderImpl!();