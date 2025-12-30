// Generated macro for AnyValueParser (trait)
macro_rules! Depcrate_builder_value_parserAnyValueParser {
() => {
// Module: crate::builder::value_parser
// Provides: {"AnyValueParser"}
// Dependencies: {}
# [doc = " A type-erased wrapper for [`TypedValueParser`]."] trait AnyValueParser : Send + Sync + 'static { fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < AnyValue , crate :: Error > ; fn parse_ref_ (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr , _source : ValueSource ,) -> Result < AnyValue , crate :: Error > { self . parse_ref (cmd , arg , value) } # [doc = " Describes the content of `AnyValue`"] fn type_id (& self) -> AnyValueId ; fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > ; fn clone_any (& self) -> Box < dyn AnyValueParser > ; }
};
}
