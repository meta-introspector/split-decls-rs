// Generated macro for impl_402 (impl)
macro_rules! Depcrate_builder_value_parserimpl_402 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_402"}
// Dependencies: {}
impl TypedValueParser for FalseyValueParser { type Value = bool ; fn parse_ref (& self , cmd : & crate :: Command , _arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (value . to_str () . ok_or_else (|| { crate :: Error :: invalid_utf8 (cmd , crate :: output :: Usage :: new (cmd) . create_usage_with_title (& []) ,) })) ; let value = if value . is_empty () { false } else { crate :: util :: str_to_bool (value) . unwrap_or (true) } ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { Some (Box :: new (Self :: possible_values ())) } }
};
}
