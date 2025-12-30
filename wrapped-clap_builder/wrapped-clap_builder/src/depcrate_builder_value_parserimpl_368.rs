// Generated macro for impl_368 (impl)
macro_rules! Depcrate_builder_value_parserimpl_368 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_368"}
// Dependencies: {}
impl TypedValueParser for StringValueParser { type Value = String ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , cmd : & crate :: Command , _arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (value . into_string () . map_err (| _ | { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; Ok (value) } }
};
}
