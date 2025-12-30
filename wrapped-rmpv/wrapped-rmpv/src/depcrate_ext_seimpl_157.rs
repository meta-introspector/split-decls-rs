// Generated macro for impl_157 (impl)
macro_rules! Depcrate_ext_seimpl_157 {
() => {
// Module: crate::ext::se
// Provides: {"impl_157"}
// Dependencies: {}
impl SerializeTupleStruct for SerializeVec { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Error > where T : Serialize { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Value , Error > { ser :: SerializeSeq :: end (self) } }
};
}
