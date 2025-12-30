// Generated macro for impl_303 (impl)
macro_rules! Depcrate_serimpl_303 {
() => {
// Module: crate::ser
// Provides: {"impl_303"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeTuple for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + ser :: Serialize > (& mut self , value : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , value) } fn end (self) -> Result < () , Error > { self . ser . write_end_collection () } }
};
}
