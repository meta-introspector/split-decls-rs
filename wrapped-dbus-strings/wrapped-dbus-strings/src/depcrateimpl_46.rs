// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl SignatureMulti { # [doc = " Splits this signature into the first and remaining parts."] # [doc = ""] # [doc = " Returns none if the signature is empty."] pub fn single (& self) -> Option < (& SignatureSingle , & SignatureMulti) > { validity :: sig_single (self . as_bytes () , 0 , 0) . map (| x | (SignatureSingle :: new_unchecked (& self [0 .. x]) , SignatureMulti :: new_unchecked (& self [x ..]))) } }
};
}
