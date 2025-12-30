// Generated macro for min_length (function)
macro_rules! Depcrate_validators_min_lengthmin_length {
() => {
// Module: crate::validators::min_length
// Provides: {"min_length"}
// Dependencies: {}
pub fn min_length < T : AsRef < str > + InputType > (value : & T , len : usize ,) -> Result < () , InputValueError < T > > { if value . as_ref () . len () >= len { Ok (()) } else { Err (format ! ("the string length is {}, must be greater than or equal to {}" , value . as_ref () . len () , len) . into ()) } }
};
}
