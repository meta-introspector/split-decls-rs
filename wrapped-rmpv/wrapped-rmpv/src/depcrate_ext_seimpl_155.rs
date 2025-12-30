// Generated macro for impl_155 (impl)
macro_rules! Depcrate_ext_seimpl_155 {
() => {
// Module: crate::ext::se
// Provides: {"impl_155"}
// Dependencies: {}
impl SerializeSeq for SerializeVec { type Ok = Value ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize { self . vec . push (to_value (value) ?) ; Ok (()) } # [inline] fn end (self) -> Result < Value , Error > { Ok (Value :: Array (self . vec)) } }
};
}
