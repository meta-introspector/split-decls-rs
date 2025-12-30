// Generated macro for impl_294 (impl)
macro_rules! Depcrate_value_deimpl_294 {
() => {
// Module: crate::value::de
// Provides: {"impl_294"}
// Dependencies: {}
impl < 'de > VariantAccess < 'de > for VariantDeserializer { type Error = Error ; fn unit_variant (self) -> Result < () , Error > { match self . value { Some (value) => Deserialize :: deserialize (value) , None => Ok (()) , } } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Error > where T : DeserializeSeed < 'de > , { match self . value { Some (value) => seed . deserialize (value) , None => Err (serde :: de :: Error :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { match self . value { Some (Value :: Array (v)) => { if v . is_empty () { visitor . visit_unit () } else { visit_array (v , visitor) } } Some (other) => Err (serde :: de :: Error :: invalid_type (other . unexpected () , & "tuple variant" ,)) , None => Err (serde :: de :: Error :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : Visitor < 'de > , { match self . value { Some (Value :: Object (v)) => v . deserialize_any (visitor) , Some (other) => Err (serde :: de :: Error :: invalid_type (other . unexpected () , & "struct variant" ,)) , None => Err (serde :: de :: Error :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) , } } }
};
}
