// Generated macro for impl_330 (impl)
macro_rules! Depcrate_hkdfimpl_330 {
() => {
// Module: crate::hkdf
// Provides: {"impl_330"}
// Dependencies: {}
impl < L : KeyType > Okm < '_ , L > { # [doc = " The `OkmLength` given to `Prk::expand()`."] # [inline] pub fn len (& self) -> & L { & self . len } # [doc = " Fills `out` with the output of the HKDF-Expand operation for the given"] # [doc = " inputs."] # [doc = ""] # [doc = " Fails if (and only if) the requested output length is larger than 255"] # [doc = " times the size of the digest algorithm's output. (This is the limit"] # [doc = " imposed by the HKDF specification due to the way HKDF's counter is"] # [doc = " constructed.)"] # [inline] pub fn fill (self , out : & mut [u8]) -> Result < () , error :: Unspecified > { fill_okm (self . prk , self . info , out , self . len_cached) } }
};
}
