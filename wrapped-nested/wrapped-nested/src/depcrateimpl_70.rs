// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'sval , Ok > StreamMap < 'sval > for Unsupported < Ok > { type Ok = Ok ; fn key_computed < V : sval :: Value > (& mut self , _ : V) -> Result { Err (Error :: invalid_value ("maps are unsupported")) } fn value_computed < V : sval :: Value > (& mut self , _ : V) -> Result { Err (Error :: invalid_value ("maps are unsupported")) } fn end (self) -> Result < Self :: Ok > { Err (Error :: invalid_value ("maps are unsupported")) } }
};
}
