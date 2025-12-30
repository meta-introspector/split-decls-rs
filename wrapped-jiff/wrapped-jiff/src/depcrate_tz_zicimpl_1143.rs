// Generated macro for impl_1143 (impl)
macro_rules! Depcrate_tz_zicimpl_1143 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1143"}
// Dependencies: {}
impl FromStr for RuleSaveSuffixP { type Err = Error ; fn from_str (suffix : & str) -> Result < RuleSaveSuffixP , Error > { match suffix { "s" => Ok (RuleSaveSuffixP :: Standard) , "d" => Ok (RuleSaveSuffixP :: Dst) , _ => Err (err ! ("unrecognized SAVE time suffix {suffix:?}")) , } } }
};
}
