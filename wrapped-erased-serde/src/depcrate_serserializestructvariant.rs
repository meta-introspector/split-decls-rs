// Generated macro for SerializeStructVariant (trait)
macro_rules! Depcrate_serSerializeStructVariant {
() => {
// Module: crate::ser
// Provides: {"SerializeStructVariant"}
// Dependencies: {}
pub trait SerializeStructVariant { fn erased_serialize_field (& mut self , key : & 'static str , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_skip_field (& mut self , key : & 'static str) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
};
}
