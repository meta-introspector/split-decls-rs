// Generated macro for impl_156 (impl)
macro_rules! Depcrate_ext_seimpl_156 {
() => {
// Module: crate::ext::se
// Provides: {"impl_156"}
// Dependencies: {}
impl SerializeTuple for SerializeVec { type Ok = Value ; type Error = Error ; # [inline] fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Value , Error > { ser :: SerializeSeq :: end (self) } }
};
}
