// Generated macro for impl_265 (impl)
macro_rules! Depcrate_serimpl_265 {
() => {
// Module: crate::ser
// Provides: {"impl_265"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for SeqSerializer < '_ > { type Ok = () ; type Error = ConfigError ; fn serialize_field < T > (& mut self , value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok > { ser :: SerializeSeq :: end (self) } }
};
}
