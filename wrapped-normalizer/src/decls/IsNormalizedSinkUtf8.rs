macro_rules! IsNormalizedSinkUtf8 {
    () => {
        # [cfg (feature = "utf8_iter")] struct IsNormalizedSinkUtf8 < 'a > { expect : & 'a [u8] , }
    };
}

IsNormalizedSinkUtf8!()