// Generated macro for impl_1138 (impl)
macro_rules! Depcrate_tz_zicimpl_1138 {
() => {
// Module: crate::tz::zic
// Provides: {"impl_1138"}
// Dependencies: {}
impl FromStr for RuleAtSuffixP { type Err = Error ; fn from_str (suffix : & str) -> Result < RuleAtSuffixP , Error > { match suffix { "w" => Ok (RuleAtSuffixP :: Wall) , "s" => Ok (RuleAtSuffixP :: Standard) , "u" | "g" | "z" => Ok (RuleAtSuffixP :: Universal) , _ => Err (err ! ("unrecognized AT time suffix {suffix:?}")) , } } }
};
}
