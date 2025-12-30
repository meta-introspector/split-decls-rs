// Generated macro for impl_414 (impl)
macro_rules! Depcrate_builder_value_parserimpl_414 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_414"}
// Dependencies: {}
impl < P , F , T > TypedValueParser for MapValueParser < P , F > where P : TypedValueParser , P :: Value : Send + Sync + Clone , F : Fn (P :: Value) -> T + Clone + Send + Sync + 'static , T : Send + Sync + Clone , { type Value = T ; fn parse_ref (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : & std :: ffi :: OsStr ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (self . parser . parse_ref (cmd , arg , value)) ; let value = (self . func) (value) ; Ok (value) } fn parse (& self , cmd : & crate :: Command , arg : Option < & crate :: Arg > , value : std :: ffi :: OsString ,) -> Result < Self :: Value , crate :: Error > { let value = ok ! (self . parser . parse (cmd , arg , value)) ; let value = (self . func) (value) ; Ok (value) } fn possible_values (& self ,) -> Option < Box < dyn Iterator < Item = crate :: builder :: PossibleValue > + '_ > > { self . parser . possible_values () } }
};
}
