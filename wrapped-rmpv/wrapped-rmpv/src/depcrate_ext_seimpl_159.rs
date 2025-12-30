// Generated macro for impl_159 (impl)
macro_rules! Depcrate_ext_seimpl_159 {
() => {
// Module: crate::ext::se
// Provides: {"impl_159"}
// Dependencies: {}
impl ser :: SerializeMap for DefaultSerializeMap { type Ok = Value ; type Error = Error ; # [inline] fn serialize_key < T : ? Sized > (& mut self , key : & T) -> Result < () , Error > where T : Serialize { self . next_key = Some (to_value (key) ?) ; Ok (()) } fn serialize_value < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : ser :: Serialize { let key = self . next_key . take () . expect ("`serialize_value` called before `serialize_key`") ; self . map . push ((key , to_value (value) ?)) ; Ok (()) } # [inline] fn end (self) -> Result < Value , Error > { Ok (Value :: Map (self . map)) } }
};
}
