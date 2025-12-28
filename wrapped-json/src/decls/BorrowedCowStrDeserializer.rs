macro_rules! BorrowedCowStrDeserializer {
    () => {
        struct BorrowedCowStrDeserializer < 'de > { value : Cow < 'de , str > , }
    };
}

BorrowedCowStrDeserializer!();