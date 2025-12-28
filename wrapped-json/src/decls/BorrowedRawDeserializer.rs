macro_rules! BorrowedRawDeserializer {
    () => {
        pub struct BorrowedRawDeserializer < 'de > { pub raw_value : Option < & 'de str > , }
    };
}

BorrowedRawDeserializer!();