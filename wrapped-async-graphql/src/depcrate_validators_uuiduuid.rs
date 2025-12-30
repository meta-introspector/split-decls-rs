// Generated macro for uuid (function)
macro_rules! Depcrate_validators_uuiduuid {
() => {
// Module: crate::validators::uuid
// Provides: {"uuid"}
// Dependencies: {}
pub fn uuid < T : AsRef < str > + InputType > (value : & T , version_option : Option < usize > ,) -> Result < () , InputValueError < T > > { match Uuid :: try_parse (value . as_ref ()) { Ok (uuid) => { if let Some (version) = version_option { if uuid . get_version_num () != version { return Err (InputValueError :: custom ("UUID version mismatch")) ; } } Ok (()) } Err (_) => Err (InputValueError :: custom ("Invalid UUID")) , } }
};
}
