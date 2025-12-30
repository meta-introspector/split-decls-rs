// Generated macro for impl_160 (impl)
macro_rules! Depcrate_ext_seimpl_160 {
() => {
// Module: crate::ext::se
// Provides: {"impl_160"}
// Dependencies: {}
impl SerializeStruct for SerializeVec { type Ok = Value ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized > (& mut self , _key : & 'static str , value : & T) -> Result < () , Error > where T : Serialize { ser :: SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Value , Error > { ser :: SerializeSeq :: end (self) } }
};
}
