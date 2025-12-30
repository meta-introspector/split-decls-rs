// Generated macro for impl_90 (impl)
macro_rules! Depcrate_de_simple_typeimpl_90 {
() => {
// Module: crate::de::simple_type
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'de > VariantAccess < 'de > for UnitOnly { type Error = DeError ; # [inline] fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { seed . deserialize (UnitDeserializer :: < Self :: Error > :: new ()) } # [inline] fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_unit () } # [inline] fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_unit () } }
};
}
