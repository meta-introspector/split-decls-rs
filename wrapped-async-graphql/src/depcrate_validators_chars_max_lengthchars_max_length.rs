// Generated macro for chars_max_length (function)
macro_rules! Depcrate_validators_chars_max_lengthchars_max_length {
() => {
// Module: crate::validators::chars_max_length
// Provides: {"chars_max_length"}
// Dependencies: {}
pub fn chars_max_length < T : AsRef < str > + InputType > (value : & T , len : usize ,) -> Result < () , InputValueError < T > > { if value . as_ref () . chars () . count () <= len { Ok (()) } else { Err (format ! ("the chars length is {}, must be less than or equal to {}" , value . as_ref () . chars () . count () , len) . into ()) } }
};
}
