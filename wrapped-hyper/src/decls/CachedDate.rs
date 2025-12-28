macro_rules! CachedDate {
    () => {
        struct CachedDate { bytes : [u8 ; DATE_VALUE_LENGTH] , pos : usize , # [cfg (feature = "http2")] header_value : HeaderValue , next_update : SystemTime , }
    };
}

CachedDate!()