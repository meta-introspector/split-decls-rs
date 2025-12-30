// Generated macro for impl_135 (impl)
macro_rules! Depcrate_encoding_rulesimpl_135 {
() => {
// Module: crate::encoding_rules
// Provides: {"impl_135"}
// Dependencies: {}
impl EncodingRules { # [doc = " Are we using Basic Encoding Rules?"] # [cfg (feature = "ber")] pub const fn is_ber (self) -> bool { matches ! (self , EncodingRules :: Ber) } # [doc = " Are we using Distinguished Encoding Rules?"] pub const fn is_der (self) -> bool { matches ! (self , EncodingRules :: Der) } }
};
}
