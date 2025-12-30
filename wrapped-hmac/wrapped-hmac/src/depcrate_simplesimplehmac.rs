// Generated macro for SimpleHmac (struct)
macro_rules! Depcrate_simpleSimpleHmac {
() => {
// Module: crate::simple
// Provides: {"SimpleHmac"}
// Dependencies: {}
# [doc = " Simplified HMAC instance able to operate over hash functions"] # [doc = " which do not expose block-level API and hash functions which"] # [doc = " process blocks lazily (e.g. BLAKE2)."] # [derive (Clone)] pub struct SimpleHmac < D : Digest + BlockSizeUser > { digest : D , opad_key : Block < D > , }
};
}
