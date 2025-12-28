macro_rules! IsNormalizedSinkStr {
    () => {
        struct IsNormalizedSinkStr < 'a > { expect : & 'a str , }
    };
}

IsNormalizedSinkStr!()