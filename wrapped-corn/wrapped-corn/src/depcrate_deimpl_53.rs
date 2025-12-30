// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deimpl_53 {
() => {
// Module: crate::de
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de > EnumAccess < 'de > for Enum < 'de > { type Error = Error ; type Variant = Variant < 'de > ; fn variant_seed < V > (self , seed : V) -> std :: result :: Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { match self . value { Value :: String (_) => { let value = seed . deserialize (& mut Deserializer :: from_value (self . value)) ? ; Ok ((value , Variant :: new (None))) } Value :: Object (obj) => { let first_pair = obj . into_iter () . next () ; if let Some (first_pair) = first_pair { let value = Value :: String (first_pair . 0) ; let tag = seed . deserialize (& mut Deserializer :: from_value (value)) ? ; Ok ((tag , Variant :: new (Some (first_pair . 1)))) } else { Err (Error :: DeserializationError ("Cannot deserialize empty object into enum" . to_string () ,)) } } _ => unreachable ! () , } } }
};
}
