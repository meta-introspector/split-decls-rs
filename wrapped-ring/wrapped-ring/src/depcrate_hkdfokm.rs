// Generated macro for Okm (struct)
macro_rules! Depcrate_hkdfOkm {
() => {
// Module: crate::hkdf
// Provides: {"Okm"}
// Dependencies: {}
# [doc = " An HKDF OKM (Output Keying Material)"] # [doc = ""] # [doc = " Intentionally not `Clone` or `Copy` as an OKM is generally only safe to"] # [doc = " use once."] # [derive (Debug)] pub struct Okm < 'a , L : KeyType > { prk : & 'a Prk , info : & 'a [& 'a [u8]] , len : L , len_cached : usize , }
};
}
