// Generated macro for impl_307 (impl)
macro_rules! Depcrate_serimpl_307 {
() => {
// Module: crate::ser
// Provides: {"impl_307"}
// Dependencies: {}
impl < W : Writer > ser :: SerializeStruct for Compound < '_ , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Error > { self . ser . serialize_with_option_mode (OptionMode :: StructField (key) , value) } fn end (self) -> Result < () , Error > { self . ser . write_end_collection () } }
};
}
