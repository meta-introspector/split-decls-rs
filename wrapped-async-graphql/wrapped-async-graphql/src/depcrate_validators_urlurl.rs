// Generated macro for url (function)
macro_rules! Depcrate_validators_urlurl {
() => {
// Module: crate::validators::url
// Provides: {"url"}
// Dependencies: {}
pub fn url < T : AsRef < str > + InputType > (value : & T) -> Result < () , InputValueError < T > > { if let Ok (true) = http :: uri :: Uri :: from_str (value . as_ref ()) . map (| uri | uri . scheme () . is_some () && uri . authority () . is_some ()) { Ok (()) } else { Err ("invalid url" . into ()) } }
};
}
