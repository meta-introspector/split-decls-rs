// Generated macro for ValueParserInner (enum)
macro_rules! Depcrate_builder_value_parserValueParserInner {
() => {
// Module: crate::builder::value_parser
// Provides: {"ValueParserInner"}
// Dependencies: {}
enum ValueParserInner { Bool , String , OsString , PathBuf , Other (Box < dyn AnyValueParser >) , }
};
}
