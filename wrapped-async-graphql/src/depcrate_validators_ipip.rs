// Generated macro for ip (function)
macro_rules! Depcrate_validators_ipip {
() => {
// Module: crate::validators::ip
// Provides: {"ip"}
// Dependencies: {}
pub fn ip < T : AsRef < str > + InputType > (value : & T) -> Result < () , InputValueError < T > > { if IpAddr :: from_str (value . as_ref ()) . is_ok () { Ok (()) } else { Err ("invalid ip" . into ()) } }
};
}
