// Generated macro for impl_27 (impl)
macro_rules! Depcrate_deimpl_27 {
() => {
// Module: crate::de
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'de , 'a , 'b , R : Read > de :: SeqAccess < 'de > for Access < 'a , 'b , R > where R :: Error : core :: fmt :: Debug , { type Error = Error < R :: Error > ; # [inline] fn next_element_seed < U : de :: DeserializeSeed < 'de > > (& mut self , seed : U ,) -> Result < Option < U :: Value > , Self :: Error > { match self . 1 { Some (0) => return Ok (None) , Some (x) => self . 1 = Some (x - 1) , None => match self . 0 . decoder . pull () ? { Header :: Break => return Ok (None) , header => self . 0 . decoder . push (header) , } , } seed . deserialize (& mut * self . 0) . map (Some) } # [inline] fn size_hint (& self) -> Option < usize > { self . 1 } }
};
}
