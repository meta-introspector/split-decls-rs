// Generated macro for min_password_strength (function)
macro_rules! Depcrate_validators_min_password_strengthmin_password_strength {
() => {
// Module: crate::validators::min_password_strength
// Provides: {"min_password_strength"}
// Dependencies: {}
pub fn min_password_strength < T : AsRef < str > + InputType > (value : & T , min_score : u8 ,) -> Result < () , InputValueError < T > > { match zxcvbn (value . as_ref () , & []) { Ok (password_strength) => { if password_strength . score () < min_score { Err ("password is too weak" . into ()) } else { Ok (()) } } Err (ZxcvbnError :: BlankPassword) => Err ("password is too weak" . into ()) , _ => Err ("error processing password strength" . into ()) , } }
};
}
