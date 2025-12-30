// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'sval , Ok > StreamSeq < 'sval > for Unsupported < Ok > { type Ok = Ok ; fn value_computed < V : sval :: Value > (& mut self , _ : V) -> Result { Err (Error :: invalid_value ("sequences are unsupported")) } fn end (self) -> Result < Self :: Ok > { Err (Error :: invalid_value ("sequences are unsupported")) } }
};
}
