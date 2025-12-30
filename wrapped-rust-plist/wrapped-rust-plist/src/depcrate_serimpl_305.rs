// Generated macro for impl_305 (impl)
macro_rules! Depcrate_serimpl_305 {
() => {
// Module: crate::ser
// Provides: {"impl_305"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeTupleVariant for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , value : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . write_end_collection () ? ; self . ser . write_end_collection () } }
};
}
