// Generated macro for impl_56 (impl)
macro_rules! Depcrate_deimpl_56 {
() => {
// Module: crate::de
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'de > VariantAccess < 'de > for Variant < 'de > { type Error = Error ; fn unit_variant (self) -> std :: result :: Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> std :: result :: Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { match self . value { Some (value) => seed . deserialize (& mut Deserializer :: from_value (value)) , None => Err (Error :: DeserializationError ("Expected value to exist" . to_string () ,)) , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> std :: result :: Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { match self . value { Some (value) if matches ! (value , Value :: Array (_)) => visitor . visit_seq (Seq :: new (value)) , _ => unreachable ! () , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> std :: result :: Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { match self . value { Some (value) if matches ! (value , Value :: Object (_)) => visitor . visit_map (Map :: new (value)) , _ => unreachable ! () , } } }
};
}
