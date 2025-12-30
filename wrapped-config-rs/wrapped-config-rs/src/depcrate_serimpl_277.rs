// Generated macro for impl_277 (impl)
macro_rules! Depcrate_serimpl_277 {
() => {
// Module: crate::ser
// Provides: {"impl_277"}
// Dependencies: {}
impl ser :: SerializeMap for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_key < T > (& mut self , _key : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn serialize_value < T > (& mut self , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
