// Generated macro for impl_136 (impl)
macro_rules! Depcrate_encoding_rulesimpl_136 {
() => {
// Module: crate::encoding_rules
// Provides: {"impl_136"}
// Dependencies: {}
impl FromStr for EncodingRules { type Err = Error ; fn from_str (s : & str) -> Result < Self , Error > { match s { # [cfg (feature = "ber")] "ber" | "BER" => Ok (EncodingRules :: Ber) , "der" | "DER" => Ok (EncodingRules :: Der) , _ => Err (ErrorKind :: EncodingRules . into ()) , } } }
};
}
