// Generated macro for impl_361 (impl)
macro_rules! Depcrate_builder_value_parserimpl_361 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_361"}
// Dependencies: {}
impl Clone for ValueParser { fn clone (& self) -> Self { Self (match & self . 0 { ValueParserInner :: Bool => ValueParserInner :: Bool , ValueParserInner :: String => ValueParserInner :: String , ValueParserInner :: OsString => ValueParserInner :: OsString , ValueParserInner :: PathBuf => ValueParserInner :: PathBuf , ValueParserInner :: Other (o) => ValueParserInner :: Other (o . clone_any ()) , }) } }
};
}
