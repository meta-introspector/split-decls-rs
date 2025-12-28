macro_rules! EncodeUtf16Producer {
    () => {
        struct EncodeUtf16Producer < 'ch > { chars : & 'ch str , }
    };
}

EncodeUtf16Producer!();