// Generated macro for impl_264 (impl)
macro_rules! Depcrate_serimpl_264 {
() => {
// Module: crate::ser
// Provides: {"impl_264"}
// Dependencies: {}
impl ser :: SerializeTuple for SeqSerializer < '_ > { type Ok = () ; type Error = ConfigError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok > { ser :: SerializeSeq :: end (self) } }
};
}
