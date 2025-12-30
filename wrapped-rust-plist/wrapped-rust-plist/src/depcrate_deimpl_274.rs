// Generated macro for impl_274 (impl)
macro_rules! Depcrate_deimpl_274 {
() => {
// Module: crate::de
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'de , 'event , I > de :: VariantAccess < 'de > for & mut Deserializer < 'event , I > where I : IntoIterator < Item = Result < Event < 'event > , Error > > , { type Error = Error ; fn unit_variant (self) -> Result < () , Error > { < () as de :: Deserialize > :: deserialize (self) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Error > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self) } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { de :: Deserializer :: deserialize_tuple (self , len , visitor) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { let name = "" ; de :: Deserializer :: deserialize_struct (self , name , fields , visitor) } }
};
}
