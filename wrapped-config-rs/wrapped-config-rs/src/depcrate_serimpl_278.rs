// Generated macro for impl_278 (impl)
macro_rules! Depcrate_serimpl_278 {
() => {
// Module: crate::ser
// Provides: {"impl_278"}
// Dependencies: {}
impl ser :: SerializeStruct for Unreachable { type Ok = String ; type Error = ConfigError ; fn serialize_field < T > (& mut self , _key : & 'static str , _value : & T) -> Result < () > where T : ? Sized + ser :: Serialize , { match * self { } } fn end (self) -> Result < Self :: Ok > { match self { } } }
};
}
