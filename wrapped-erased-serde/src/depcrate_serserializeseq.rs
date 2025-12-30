// Generated macro for SerializeSeq (trait)
macro_rules! Depcrate_serSerializeSeq {
() => {
// Module: crate::ser
// Provides: {"SerializeSeq"}
// Dependencies: {}
pub trait SerializeSeq { fn erased_serialize_element (& mut self , value : & dyn Serialize) -> Result < () , ErrorImpl > ; fn erased_end (& mut self) ; }
};
}
