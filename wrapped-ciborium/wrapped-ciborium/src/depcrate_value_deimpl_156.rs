// Generated macro for impl_156 (impl)
macro_rules! Depcrate_value_deimpl_156 {
() => {
// Module: crate::value::de
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'de > de :: VariantAccess < 'de > for Deserializer < & Value > { type Error = Error ; # [inline] fn unit_variant (self) -> Result < () , Self :: Error > { match self . 0 { Value :: Null => Ok (()) , _ => Err (de :: Error :: invalid_type (self . 0 . into () , & "unit")) , } } # [inline] fn newtype_variant_seed < U : de :: DeserializeSeed < 'de > > (self , seed : U ,) -> Result < U :: Value , Self :: Error > { seed . deserialize (self) } # [inline] fn tuple_variant < V : de :: Visitor < 'de > > (self , _len : usize , visitor : V ,) -> Result < V :: Value , Self :: Error > { self . deserialize_seq (visitor) } # [inline] fn struct_variant < V : de :: Visitor < 'de > > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > { self . deserialize_map (visitor) } }
};
}
