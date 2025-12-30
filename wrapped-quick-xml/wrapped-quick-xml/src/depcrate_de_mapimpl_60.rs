// Generated macro for impl_60 (impl)
macro_rules! Depcrate_de_mapimpl_60 {
() => {
// Module: crate::de::map
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'de , 'd , R , E > de :: VariantAccess < 'de > for ElementDeserializer < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; fn unit_variant (self) -> Result < () , Self :: Error > { self . de . read_to_end (self . start . name ()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { seed . deserialize (self) } # [inline] fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { self . deserialize_tuple (len , visitor) } # [inline] fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { self . deserialize_struct ("" , fields , visitor) } }
};
}
