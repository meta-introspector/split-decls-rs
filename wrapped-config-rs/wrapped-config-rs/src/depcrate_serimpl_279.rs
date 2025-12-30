// Generated macro for impl_279 (impl)
macro_rules! Depcrate_serimpl_279 {
() => {
// Module: crate::ser
// Provides: {"impl_279"}
// Dependencies: {}
impl ser :: SerializeStructVariant for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_field < T > (& mut self , _key : & 'static str , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
