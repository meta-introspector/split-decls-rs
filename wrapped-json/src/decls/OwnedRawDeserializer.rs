macro_rules! OwnedRawDeserializer {
    () => {
        pub struct OwnedRawDeserializer { pub raw_value : Option < String > , }
    };
}

OwnedRawDeserializer!();