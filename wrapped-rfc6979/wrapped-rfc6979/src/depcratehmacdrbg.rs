// Generated macro for HmacDrbg (struct)
macro_rules! DepcrateHmacDrbg {
() => {
// Module: crate
// Provides: {"HmacDrbg"}
// Dependencies: {}
# [doc = " Internal implementation of `HMAC_DRBG` as described in NIST SP800-90A."] # [doc = ""] # [doc = " <https://csrc.nist.gov/publications/detail/sp/800-90a/rev-1/final>"] # [doc = ""] # [doc = " This is a HMAC-based deterministic random bit generator used compute a"] # [doc = " deterministic ephemeral scalar `k`."] pub struct HmacDrbg < D > where D : EagerHash , { # [doc = " HMAC key `K` (see RFC 6979 Section 3.2.c)"] k : HmacReset < D > , # [doc = " Chaining value `V` (see RFC 6979 Section 3.2.c)"] v : Array < u8 , < D :: Core as OutputSizeUser > :: OutputSize > , }
};
}
