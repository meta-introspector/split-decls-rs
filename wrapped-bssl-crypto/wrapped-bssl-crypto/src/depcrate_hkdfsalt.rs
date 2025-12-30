// Generated macro for Salt (enum)
macro_rules! Depcrate_hkdfSalt {
() => {
// Module: crate::hkdf
// Provides: {"Salt"}
// Dependencies: {}
# [doc = " HKDF's optional salt values. See <https://datatracker.ietf.org/doc/html/rfc5869#section-3.1>"] pub enum Salt < 'a > { # [doc = " No salt."] None , # [doc = " An explicit salt. Note that an empty value here is interpreted the same"] # [doc = " as if passing `None`."] NonEmpty (& 'a [u8]) , }
};
}
