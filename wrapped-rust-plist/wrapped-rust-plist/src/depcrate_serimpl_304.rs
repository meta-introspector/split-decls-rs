// Generated macro for impl_304 (impl)
macro_rules! Depcrate_serimpl_304 {
() => {
// Module: crate::ser
// Provides: {"impl_304"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeTupleStruct for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , value : & T) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: Explicit , value) } fn end (self) -> Result < () , Error > { self . ser . write_end_collection () } }
};
}
