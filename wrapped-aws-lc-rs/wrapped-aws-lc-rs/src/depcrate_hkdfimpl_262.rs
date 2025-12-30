// Generated macro for impl_262 (impl)
macro_rules! Depcrate_hkdfimpl_262 {
() => {
// Module: crate::hkdf
// Provides: {"impl_262"}
// Dependencies: {}
impl < L : KeyType > Okm < '_ , L > { # [doc = " The `OkmLength` given to `Prk::expand()`."] # [inline] pub fn len (& self) -> & L { & self . len } # [doc = " Fills `out` with the output of the HKDF-Expand operation for the given"] # [doc = " inputs."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if the requested output length differs from the length specified by"] # [doc = " `L: KeyType`."] # [inline] pub fn fill (self , out : & mut [u8]) -> Result < () , Unspecified > { if out . len () != self . len . len () { return Err (Unspecified) ; } self . prk . mode . fill (self . prk . algorithm , out , & self . info_bytes [.. self . info_len]) ? ; Ok (()) } }
};
}
