// Generated macro for chars_min_length (function)
macro_rules! Depcrate_validators_chars_min_lengthchars_min_length {
() => {
// Module: crate::validators::chars_min_length
// Provides: {"chars_min_length"}
// Dependencies: {}
pub fn chars_min_length < T : AsRef < str > + InputType > (value : & T , len : usize ,) -> Result < () , InputValueError < T > > { if value . as_ref () . chars () . count () >= len { Ok (()) } else { Err (format ! ("the chars length is {}, must be greater than or equal to {}" , value . as_ref () . chars () . count () , len) . into ()) } }
};
}
