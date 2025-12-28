macro_rules! PunycodeCaller {
    () => {
        # [doc = " Marker for internal vs. external caller to retain old API behavior"] # [doc = " while tweaking behavior for internal callers."] # [doc = ""] # [doc = " External callers need overflow checks when encoding, but internal"] # [doc = " callers don't, because `PUNYCODE_ENCODE_MAX_INPUT_LENGTH` is set"] # [doc = " to 1000, and per RFC 3492 section 6.4, the integer variable does"] # [doc = " not need to be able to represent values larger than"] # [doc = " (char::MAX - INITIAL_N) * (PUNYCODE_ENCODE_MAX_INPUT_LENGTH + 1),"] # [doc = " which is less than u32::MAX."] # [doc = ""] # [doc = " External callers need to handle upper-case ASCII when decoding,"] # [doc = " but internal callers don't, because the internal code calls the"] # [doc = " decoder only with lower-case inputs."] pub (crate) trait PunycodeCaller { const EXTERNAL_CALLER : bool ; }
    };
}

PunycodeCaller!();