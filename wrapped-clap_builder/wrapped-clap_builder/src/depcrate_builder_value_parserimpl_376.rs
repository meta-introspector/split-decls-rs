// Generated macro for impl_376 (impl)
macro_rules! Depcrate_builder_value_parserimpl_376 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_376"}
// Dependencies: {}
impl TypedValueParser for PathBufValueParser { type Value = std :: path :: PathBuf ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { TypedValueParser :: parse (self , cmd , arg , value . to_owned ()) } fn parse (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { if value . is_empty () { return Err (crate :: Error :: empty_value (cmd , & [] , arg . map (ToString :: to_string) . unwrap_or_else (| | "..." . to_owned ()) ,)) ; } Ok (Self :: Value :: from (value)) } }
};
}
