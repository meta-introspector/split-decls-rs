macro_rules! PUNYCODE_DECODE_MAX_INPUT_LENGTH {
    () => {
        # [doc = " ICU4C-compatible constraint."] # [doc = " https://unicode-org.atlassian.net/browse/ICU-13727"] const PUNYCODE_DECODE_MAX_INPUT_LENGTH : usize = 2000 ;
    };
}

PUNYCODE_DECODE_MAX_INPUT_LENGTH!()