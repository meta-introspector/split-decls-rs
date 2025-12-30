// Generated macro for SerializeMap (trait)
macro_rules! Depcrate_serSerializeMap {
() => {
// Module: crate::ser
// Provides: {"SerializeMap"}
// Dependencies: {}
pub trait SerializeMap { fn erased_serialize_key (& mut self , key : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_serialize_value (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_serialize_entry (& mut self , key : & dyn Serialize , value : & dyn Serialize ,) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
};
}
