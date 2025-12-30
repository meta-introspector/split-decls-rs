// Generated macro for impl_107 (impl)
macro_rules! Depcrate_de_textimpl_107 {
() => {
// Module: crate::de::text
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'de > EnumAccess < 'de > for TextDeserializer < 'de > { type Error = DeError ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let name = seed . deserialize (BorrowedStrDeserializer :: < DeError > :: new (TEXT_KEY)) ? ; Ok ((name , self)) } }
};
}
