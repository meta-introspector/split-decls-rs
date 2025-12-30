// Generated macro for impl_36 (impl)
macro_rules! Depcrate_deimpl_36 {
() => {
// Module: crate::de
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'de , 'a , R > de :: SeqAccess < 'de > for IndefiniteSeqAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : de :: DeserializeSeed < 'de > , { match self . de . peek () ? { Some (0xff) => return Ok (None) , Some (_) => { } None => return Err (self . de . error (ErrorCode :: EofWhileParsingArray)) , } let value = seed . deserialize (& mut * self . de) ? ; Ok (Some (value)) } }
};
}
