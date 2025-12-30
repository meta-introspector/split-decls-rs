// Generated macro for impl_90 (impl)
macro_rules! Depcrate_tagimpl_90 {
() => {
// Module: crate::tag
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'de , D : de :: Deserializer < 'de > > de :: VariantAccess < 'de > for TagAccess < D > { type Error = D :: Error ; # [inline] fn unit_variant (self) -> Result < () , Self :: Error > { Err (Self :: Error :: custom ("expected tag")) } # [inline] fn newtype_variant_seed < U : de :: DeserializeSeed < 'de > > (mut self , seed : U ,) -> Result < U :: Value , Self :: Error > { seed . deserialize (self . parent . take () . unwrap ()) } # [inline] fn tuple_variant < V : de :: Visitor < 'de > > (self , _len : usize , visitor : V ,) -> Result < V :: Value , Self :: Error > { visitor . visit_seq (self) } # [inline] fn struct_variant < V : de :: Visitor < 'de > > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Self :: Error > { Err (Self :: Error :: custom ("expected tag")) } }
};
}
