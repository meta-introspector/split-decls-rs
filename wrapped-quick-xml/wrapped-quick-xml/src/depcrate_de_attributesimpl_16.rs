// Generated macro for impl_16 (impl)
macro_rules! Depcrate_de_attributesimpl_16 {
() => {
// Module: crate::de::attributes
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'de > MapAccess < 'de > for AttributesDeserializer < 'de > { type Error = DeError ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : DeserializeSeed < 'de > , { debug_assert_eq ! (self . value , None) ; match self . iter . next () { None => Ok (None) , Some (Ok (attr)) => { self . value = Some (attr . value) ; self . key_buf . clear () ; self . key_buf . push_str (self . prefix) ; let de = QNameDeserializer :: from_attr (attr . key , self . iter . decoder () , & mut self . key_buf) ? ; seed . deserialize (de) . map (Some) } Some (Err (err)) => Err (Error :: custom (err)) , } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : DeserializeSeed < 'de > , { match self . value . take () { Some (value) => { let de = SimpleTypeDeserializer :: from_part (& value , 0 .. value . len () , self . iter . decoder ()) ; seed . deserialize (de) } None => Err (DeError :: KeyNotRead) , } } }
};
}
