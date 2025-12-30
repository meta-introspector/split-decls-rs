// Generated macro for impl_273 (impl)
macro_rules! Depcrate_serimpl_273 {
() => {
// Module: crate::ser
// Provides: {"impl_273"}
// Dependencies: {}
impl ser :: SerializeSeq for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_element < T > (& mut self , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
