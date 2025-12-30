// Generated macro for impl_42 (impl)
macro_rules! Depcrate_deimpl_42 {
() => {
// Module: crate::de
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'de , 'a , R > de :: MapAccess < 'de > for IndefiniteMapAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : de :: DeserializeSeed < 'de > , { match self . de . peek () ? { Some (_byte @ 0x00 ..= 0x1b) if ! self . accept_packed => { return Err (self . de . error (ErrorCode :: WrongStructFormat)) } Some (_byte @ 0x60 ..= 0x7f) if ! self . accept_named => { return Err (self . de . error (ErrorCode :: WrongStructFormat)) } Some (0xff) => return Ok (None) , Some (_) => { } None => return Err (self . de . error (ErrorCode :: EofWhileParsingMap)) , } let value = seed . deserialize (& mut * self . de) ? ; Ok (Some (value)) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (& mut * self . de) } }
};
}
