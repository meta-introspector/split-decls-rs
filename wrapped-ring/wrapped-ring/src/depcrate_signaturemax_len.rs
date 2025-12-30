// Generated macro for MAX_LEN (const)
macro_rules! Depcrate_signatureMAX_LEN {
() => {
// Module: crate::signature
// Provides: {"MAX_LEN"}
// Dependencies: {}
# [doc = " The longest signature is an ASN.1 P-384 signature where *r* and *s* are of"] # [doc = " maximum length with the leading high bit set on each. Then each component"] # [doc = " will have a tag, a one-byte length, and a one-byte “I'm not negative”"] # [doc = " prefix, and the outer sequence will have a two-byte length."] pub (crate) const MAX_LEN : usize = 1 + 2 + (2 * (1 + 1 + 1 + ec :: SCALAR_MAX_BYTES)) ;
};
}
