// Generated macro for impl_372 (impl)
macro_rules! Depcrate_builder_value_parserimpl_372 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_372"}
// Dependencies: {}
impl TypedValueParser for OsStringValueParser { type Value = std :: ffi :: OsString ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , _cmd : & crate :: Command , _arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { Ok (value) } }
};
}
