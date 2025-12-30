// Generated macro for impl_152 (impl)
macro_rules! Depcrate_value_deimpl_152 {
() => {
// Module: crate::value::de
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a , 'de , T : Iterator < Item = & 'a Value > > de :: SeqAccess < 'de > for Deserializer < T > { type Error = Error ; # [inline] fn next_element_seed < U : de :: DeserializeSeed < 'de > > (& mut self , seed : U ,) -> Result < Option < U :: Value > , Self :: Error > { match self . 0 . next () { None => Ok (None) , Some (v) => seed . deserialize (Deserializer (v)) . map (Some) , } } }
};
}
