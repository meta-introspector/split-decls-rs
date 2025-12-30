// Generated macro for impl_34 (impl)
macro_rules! Depcrate_deimpl_34 {
() => {
// Module: crate::de
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'de > de :: EnumAccess < 'de > for Enum < 'de > { type Error = Error ; type Variant = Variant < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) > where V : de :: DeserializeSeed < 'de > , { let span = self . pair . as_span () ; let mut res = (move | | match self . pair . as_rule () { Rule :: string => { let tag = seed . deserialize (& mut Deserializer :: from_pair (self . pair)) ? ; Ok ((tag , Variant { pair : None })) } Rule :: object => { let mut pairs = self . pair . into_inner () ; if let Some (tag_pair) = pairs . next () { let tag = seed . deserialize (& mut Deserializer :: from_pair (tag_pair)) ? ; Ok ((tag , Variant { pair : pairs . next () })) } else { Err (de :: Error :: custom ("expected a nonempty object")) } } _ => Err (de :: Error :: custom ("expected a string or an object")) , }) () ; error :: set_location (& mut res , & span) ; res } }
};
}
