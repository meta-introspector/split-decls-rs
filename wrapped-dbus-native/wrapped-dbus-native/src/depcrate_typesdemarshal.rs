// Generated macro for Demarshal (trait)
macro_rules! Depcrate_typesDemarshal {
() => {
// Module: crate::types
// Provides: {"Demarshal"}
// Dependencies: {}
pub trait Demarshal < 'a > : Marshal + Sized { fn read_buf (_ : & mut DemarshalState < 'a >) -> Result < Self , DemarshalError > ; }
};
}
