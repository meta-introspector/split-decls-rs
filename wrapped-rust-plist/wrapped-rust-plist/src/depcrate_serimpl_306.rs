// Generated macro for impl_306 (impl)
macro_rules! Depcrate_serimpl_306 {
() => {
// Module: crate::ser
// Provides: {"impl_306"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeMap for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_key < T : ? Sized + ser :: Serialize > (& mut self , key : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , key) } fn serialize_value < T : ? Sized + ser :: Serialize > (& mut self , value : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . write_end_collection () } }
};
}
