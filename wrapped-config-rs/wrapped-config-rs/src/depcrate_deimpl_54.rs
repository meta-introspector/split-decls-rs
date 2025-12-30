// Generated macro for impl_54 (impl)
macro_rules! Depcrate_deimpl_54 {
() => {
// Module: crate::de
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de > de :: VariantAccess < 'de > for EnumAccess { type Error = ConfigError ; fn unit_variant (self) -> Result < () > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value > where T : de :: DeserializeSeed < 'de > , { match self . value . kind { ValueKind :: Table (t) => seed . deserialize (t . into_iter () . next () . unwrap () . 1) , _ => unreachable ! () , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { match self . value . kind { ValueKind :: Table (t) => { de :: Deserializer :: deserialize_seq (t . into_iter () . next () . unwrap () . 1 , visitor) } _ => unreachable ! () , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { match self . value . kind { ValueKind :: Table (t) => { de :: Deserializer :: deserialize_map (t . into_iter () . next () . unwrap () . 1 , visitor) } _ => unreachable ! () , } } }
};
}
