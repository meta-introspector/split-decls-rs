// Generated macro for from_stream (function)
macro_rules! Depcrate_defrom_stream {
() => {
// Module: crate::de
// Provides: {"from_stream"}
// Dependencies: {}
pub (crate) fn from_stream < 'event , T : de :: DeserializeOwned > (stream : impl IntoIterator < Item = Result < Event < 'event > , Error > > ,) -> Result < T , Error > { let mut de = Deserializer :: new (stream) ; let value = de :: Deserialize :: deserialize (& mut de) ? ; if let Some (event) = de . events . next () . transpose () ? { return Err (ErrorKind :: ExpectedEndOfEventStream { found : EventKind :: of_event (& event) , } . without_position ()) ; } Ok (value) }
};
}
