macro_rules! IsNormalizedSinkUtf16 {
    () => {
        # [cfg (feature = "utf16_iter")] struct IsNormalizedSinkUtf16 < 'a > { expect : & 'a [u16] , }
    };
}

IsNormalizedSinkUtf16!();