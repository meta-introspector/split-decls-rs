// Generated macro for email (function)
macro_rules! Depcrate_validators_emailemail {
() => {
// Module: crate::validators::email
// Provides: {"email"}
// Dependencies: {}
pub fn email < T : AsRef < str > + InputType > (value : & T) -> Result < () , InputValueError < T > > { if is_valid_email (value . as_ref ()) { Ok (()) } else { Err ("invalid email" . into ()) } }
};
}
