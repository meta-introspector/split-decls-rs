// Generated macro for impl_153 (impl)
macro_rules! Depcrate_value_deimpl_153 {
() => {
// Module: crate::value::de
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , 'de , T : Iterator < Item = & 'a (Value , Value) > > de :: MapAccess < 'de > for Deserializer < Peekable < T > > { type Error = Error ; # [inline] fn next_key_seed < K : de :: DeserializeSeed < 'de > > (& mut self , seed : K ,) -> Result < Option < K :: Value > , Self :: Error > { match self . 0 . peek () { None => Ok (None) , Some (x) => Ok (Some (seed . deserialize (Deserializer (& x . 0)) ?)) , } } # [inline] fn next_value_seed < V : de :: DeserializeSeed < 'de > > (& mut self , seed : V ,) -> Result < V :: Value , Self :: Error > { seed . deserialize (Deserializer (& self . 0 . next () . unwrap () . 1)) } # [inline] fn next_entry_seed < K : de :: DeserializeSeed < 'de > , V : de :: DeserializeSeed < 'de > > (& mut self , kseed : K , vseed : V ,) -> Result < Option < (K :: Value , V :: Value) > , Self :: Error > { match self . 0 . next () { Some ((k , v)) => Ok (Some ((kseed . deserialize (Deserializer (k)) ? , vseed . deserialize (Deserializer (v)) ? ,))) , None => Ok (None) , } } # [inline] fn size_hint (& self) -> Option < usize > { match self . 0 . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
};
}
