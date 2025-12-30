// Generated macro for regex (function)
macro_rules! Depcrate_validators_regexregex {
() => {
// Module: crate::validators::regex
// Provides: {"regex"}
// Dependencies: {}
pub fn regex < T : AsRef < str > + InputType > (value : & T , regex : & 'static str ,) -> Result < () , InputValueError < T > > { if let Ok (true) = Regex :: new (regex) . map (| re | re . is_match (value . as_ref ())) { Ok (()) } else { Err (format_args ! ("value doesn't match expected format '{}'" , regex) . into ()) } }
};
}
