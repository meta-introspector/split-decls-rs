// Generated macro for impl_28 (impl)
macro_rules! Depcrate_deimpl_28 {
() => {
// Module: crate::de
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'de , 'a , 'b , R : Read > de :: MapAccess < 'de > for Access < 'a , 'b , R > where R :: Error : core :: fmt :: Debug , { type Error = Error < R :: Error > ; # [inline] fn next_key_seed < K : de :: DeserializeSeed < 'de > > (& mut self , seed : K ,) -> Result < Option < K :: Value > , Self :: Error > { match self . 1 { Some (0) => return Ok (None) , Some (x) => self . 1 = Some (x - 1) , None => match self . 0 . decoder . pull () ? { Header :: Break => return Ok (None) , header => self . 0 . decoder . push (header) , } , } seed . deserialize (& mut * self . 0) . map (Some) } # [inline] fn next_value_seed < V : de :: DeserializeSeed < 'de > > (& mut self , seed : V ,) -> Result < V :: Value , Self :: Error > { seed . deserialize (& mut * self . 0) } # [inline] fn size_hint (& self) -> Option < usize > { self . 1 } }
};
}
