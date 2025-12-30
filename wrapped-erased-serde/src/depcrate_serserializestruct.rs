// Generated macro for SerializeStruct (trait)
macro_rules! Depcrate_serSerializeStruct {
() => {
// Module: crate::ser
// Provides: {"SerializeStruct"}
// Dependencies: {}
pub trait SerializeStruct { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
};
}
