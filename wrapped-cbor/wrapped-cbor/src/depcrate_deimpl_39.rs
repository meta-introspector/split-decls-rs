// Generated macro for impl_39 (impl)
macro_rules! Depcrate_deimpl_39 {
() => {
// Module: crate::de
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'de , 'a , R > de :: MapAccess < 'de > for MapAccess < 'a , R > where R : Read < 'de > , { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : de :: DeserializeSeed < 'de > , { if * self . len == 0 { return Ok (None) ; } * self . len -= 1 ; match self . de . peek () ? { Some (_byte @ 0x00 ..= 0x1b) if ! self . accept_packed => { return Err (self . de . error (ErrorCode :: WrongStructFormat)) ; } Some (_byte @ 0x60 ..= 0x7f) if ! self . accept_named => { return Err (self . de . error (ErrorCode :: WrongStructFormat)) ; } _ => { } } ; let value = seed . deserialize (& mut * self . de) ? ; Ok (Some (value)) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : de :: DeserializeSeed < 'de > , { seed . deserialize (& mut * self . de) } fn size_hint (& self) -> Option < usize > { Some (* self . len) } }
};
}
