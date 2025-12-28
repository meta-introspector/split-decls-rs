macro_rules! deps {
    () => {
        BorrowedSerdeDecoder!();
        Decode!();
        DecoderImpl!();
        DecodeError!();
        SliceReader!();
        Config!();
    };
}

macro_rules! seed_decode_from_slice {
    () => {
        deps!();
        # [doc = " Decode a borrowed type from the given slice using a seed. Some parts of the decoded type are expected to be referring to the given slice"] pub fn seed_decode_from_slice < 'de , D , C > (seed : D , slice : & 'de [u8] , config : C ,) -> Result < (D :: Value , usize) , DecodeError > where D : DeserializeSeed < 'de > , C : Config , { let mut serde_decoder = BorrowedSerdeDecoder :: < DecoderImpl < SliceReader < 'de > , C , () > > :: from_slice (slice , config , ()) ; let result = seed . deserialize (serde_decoder . as_deserializer ()) ? ; let bytes_read = slice . len () - serde_decoder . de . borrow_reader () . slice . len () ; Ok ((result , bytes_read)) }
    };
}

seed_decode_from_slice!()