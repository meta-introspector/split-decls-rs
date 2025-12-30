// Generated macro for impl_276 (impl)
macro_rules! Depcrate_serimpl_276 {
() => {
// Module: crate::ser
// Provides: {"impl_276"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_field < T > (& mut self , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
