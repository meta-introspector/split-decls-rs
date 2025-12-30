// Generated macro for impl_60 (impl)
macro_rules! Depcrate_deserializerimpl_60 {
() => {
// Module: crate::deserializer
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , 'de : 'a , T : DeRecord < 'de > > VariantAccess < 'de > for & 'a mut DeRecordWrap < T > { type Error = DeserializeError ; fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < U : DeserializeSeed < 'de > > (self , _seed : U ,) -> Result < U :: Value , Self :: Error > { let unexp = Unexpected :: UnitVariant ; Err (DeserializeError :: invalid_type (unexp , & "newtype variant")) } fn tuple_variant < V : Visitor < 'de > > (self , _len : usize , _visitor : V ,) -> Result < V :: Value , Self :: Error > { let unexp = Unexpected :: UnitVariant ; Err (DeserializeError :: invalid_type (unexp , & "tuple variant")) } fn struct_variant < V : Visitor < 'de > > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Self :: Error > { let unexp = Unexpected :: UnitVariant ; Err (DeserializeError :: invalid_type (unexp , & "struct variant")) } }
};
}
