// Generated macro for impl_417 (impl)
macro_rules! Depcrate_builder_value_parserimpl_417 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_417"}
// Dependencies: {}
impl < P , F , T , E > TypedValueParser for TryMapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> Result < T , E > + Clone + Send + Sync + 'static , T : Send + Sync + Clone , E : Into < Box < dyn std :: error :: Error + Send + Sync + 'static > > , { type Value = T ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let mid_value = ok ! (self . parser . parse_ref (cmd , arg , value)) ; let value = ok ! ((self . func) (mid_value) . map_err (| e | { let arg = arg . map (| a | a . to_string ()) . unwrap_or_else (|| "..." . to_owned ()) ; crate :: Error :: value_validation (arg , value . to_string_lossy () . into_owned () , e . into ()) . with_cmd (cmd) })) ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { self . parser . possible_values () } }
};
}
