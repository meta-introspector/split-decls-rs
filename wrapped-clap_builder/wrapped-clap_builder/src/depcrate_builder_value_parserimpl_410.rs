// Generated macro for impl_410 (impl)
macro_rules! Depcrate_builder_value_parserimpl_410 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_410"}
// Dependencies: {}
impl TypedValueParser for NonEmptyStringValueParser { type Value = String ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { if value . is_empty () { return Err (crate :: Error :: empty_value (cmd , & [] , arg . map (ToString :: to_string) . unwrap_or_else (| | "..." . to_owned ()) ,)) ; } let value = ok ! (value . to_str () . ok_or_else (|| { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; Ok (value . to_owned ()) } }
};
}
