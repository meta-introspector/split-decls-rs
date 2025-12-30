// Generated macro for tests (module)
macro_rules! Depcrate_encoding_rulestests {
() => {
// Module: crate::encoding_rules
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use super :: EncodingRules ; # [cfg (feature = "alloc")] # [test] fn display () { use alloc :: string :: ToString ; # [cfg (feature = "ber")] assert_eq ! (EncodingRules :: Ber . to_string () , "BER") ; assert_eq ! (EncodingRules :: Der . to_string () , "DER") ; } # [test] fn parse () { # [cfg (feature = "ber")] assert_eq ! (EncodingRules :: Ber , "ber" . parse () . unwrap ()) ; # [cfg (feature = "ber")] assert_eq ! (EncodingRules :: Ber , "BER" . parse () . unwrap ()) ; assert_eq ! (EncodingRules :: Der , "der" . parse () . unwrap ()) ; assert_eq ! (EncodingRules :: Der , "DER" . parse () . unwrap ()) ; assert ! ("CER" . parse ::< EncodingRules > () . is_err ()) ; } }
};
}
