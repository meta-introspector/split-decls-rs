// Generated macro for impl_137 (impl)
macro_rules! Depcrate_encoding_rulesimpl_137 {
() => {
// Module: crate::encoding_rules
// Provides: {"impl_137"}
// Dependencies: {}
impl fmt :: Display for EncodingRules { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { # [cfg (feature = "ber")] Self :: Ber => "BER" , Self :: Der => "DER" , }) } }
};
}
