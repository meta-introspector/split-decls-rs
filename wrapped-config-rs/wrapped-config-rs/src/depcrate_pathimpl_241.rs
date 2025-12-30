// Generated macro for impl_241 (impl)
macro_rules! Depcrate_pathimpl_241 {
() => {
// Module: crate::path
// Provides: {"impl_241"}
// Dependencies: {}
impl FromStr for Expression { type Err = ConfigError ; fn from_str (s : & str) -> Result < Self > { parser :: from_str (s) . map_err (| e | ConfigError :: PathParse { cause : Box :: new (ParseError :: new (e)) , }) } }
};
}
