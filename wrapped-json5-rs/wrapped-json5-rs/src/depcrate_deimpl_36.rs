// Generated macro for impl_36 (impl)
macro_rules! Depcrate_deimpl_36 {
() => {
// Module: crate::de
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'de , 'a > de :: VariantAccess < 'de > for Variant < 'de > { type Error = Error ; fn unit_variant (self) -> Result < () > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (& mut Deserializer :: from_pair (self . pair . unwrap ())) } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { match self . pair { Some (pair) => match pair . as_rule () { Rule :: array => visitor . visit_seq (Seq :: new (pair)) , _ => Err (de :: Error :: custom ("expected an array")) , } , None => Err (de :: Error :: custom ("expected an array")) , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { match self . pair { Some (pair) => match pair . as_rule () { Rule :: object => visitor . visit_map (Map :: new (pair)) , _ => Err (de :: Error :: custom ("expected an object")) , } , None => Err (de :: Error :: custom ("expected an object")) , } } }
};
}
