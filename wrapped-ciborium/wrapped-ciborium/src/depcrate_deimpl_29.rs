// Generated macro for impl_29 (impl)
macro_rules! Depcrate_deimpl_29 {
() => {
// Module: crate::de
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'de , 'a , 'b , R : Read > de :: EnumAccess < 'de > for Access < 'a , 'b , R > where R :: Error : core :: fmt :: Debug , { type Error = Error < R :: Error > ; type Variant = Self ; # [inline] fn variant_seed < V : de :: DeserializeSeed < 'de > > (self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > { let variant = seed . deserialize (& mut * self . 0) ? ; Ok ((variant , self)) } }
};
}
