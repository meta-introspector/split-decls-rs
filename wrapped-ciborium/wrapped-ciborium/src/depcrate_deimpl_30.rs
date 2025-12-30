// Generated macro for impl_30 (impl)
macro_rules! Depcrate_deimpl_30 {
() => {
// Module: crate::de
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'de , 'a , 'b , R : Read > de :: VariantAccess < 'de > for Access < 'a , 'b , R > where R :: Error : core :: fmt :: Debug , { type Error = Error < R :: Error > ; # [inline] fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } # [inline] fn newtype_variant_seed < U : de :: DeserializeSeed < 'de > > (self , seed : U ,) -> Result < U :: Value , Self :: Error > { seed . deserialize (& mut * self . 0) } # [inline] fn tuple_variant < V : de :: Visitor < 'de > > (self , _len : usize , visitor : V ,) -> Result < V :: Value , Self :: Error > { self . 0 . deserialize_any (visitor) } # [inline] fn struct_variant < V : de :: Visitor < 'de > > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > { self . 0 . deserialize_any (visitor) } }
};
}
