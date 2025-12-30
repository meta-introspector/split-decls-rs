// Generated macro for impl_274 (impl)
macro_rules! Depcrate_serimpl_274 {
() => {
// Module: crate::ser
// Provides: {"impl_274"}
// Dependencies: {}
impl ser :: SerializeTuple for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_element < T > (& mut self , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
