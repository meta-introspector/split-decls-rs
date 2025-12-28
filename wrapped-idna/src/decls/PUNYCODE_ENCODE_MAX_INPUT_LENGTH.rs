macro_rules! PUNYCODE_ENCODE_MAX_INPUT_LENGTH {
    () => {
        # [doc = " ICU4C-compatible constraint. (Note: ICU4C measures"] # [doc = " UTF-16 and we measure UTF-32. This means that we"] # [doc = " allow longer non-BMP inputs. For this implementation,"] # [doc = " the denial-of-service scaling does not depend on BMP vs."] # [doc = " non-BMP: only the scalar values matter.)"] # [doc = ""] # [doc = " https://unicode-org.atlassian.net/browse/ICU-13727"] const PUNYCODE_ENCODE_MAX_INPUT_LENGTH : usize = 1000 ;
    };
}

PUNYCODE_ENCODE_MAX_INPUT_LENGTH!()