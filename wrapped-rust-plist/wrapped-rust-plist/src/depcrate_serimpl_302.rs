// Generated macro for impl_302 (impl)
macro_rules! Depcrate_serimpl_302 {
() => {
// Module: crate::ser
// Provides: {"impl_302"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeSeq for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized + ser :: Serialize > (& mut self , value : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . write_end_collection () } }
};
}
