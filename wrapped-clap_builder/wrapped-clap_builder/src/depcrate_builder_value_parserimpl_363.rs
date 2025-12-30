// Generated macro for impl_363 (impl)
macro_rules! Depcrate_builder_value_parserimpl_363 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_363"}
// Dependencies: {}
impl < T , P > AnyValueParser for P where T : std :: any :: Any + Clone + Send + Sync + 'static , P : TypedValueParser < Value = T > , { fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < AnyValue , crate :: Error > { let value = ok ! (TypedValueParser :: parse_ref (self , cmd , arg , value)) ; Ok (AnyValue :: new (value)) } fn parse_ref_ (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr , source : ValueSource ,) -> Result < AnyValue , crate :: Error > { let value = ok ! (TypedValueParser :: parse_ref_ (self , cmd , arg , value , source)) ; Ok (AnyValue :: new (value)) } fn type_id (& self) -> AnyValueId { AnyValueId :: of :: < T > () } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { P :: possible_values (self) } fn clone_any (& self) -> Box < dyn AnyValueParser > { Box :: new (self . clone ()) } }
};
}
