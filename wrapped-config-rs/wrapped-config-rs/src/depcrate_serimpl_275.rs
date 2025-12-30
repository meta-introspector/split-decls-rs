// Generated macro for impl_275 (impl)
macro_rules! Depcrate_serimpl_275 {
() => {
// Module: crate::ser
// Provides: {"impl_275"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_field < T > (& mut self , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
