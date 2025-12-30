// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'sval , Ok > StreamRecord < 'sval > for Unsupported < Ok > { type Ok = Ok ; fn value_computed < V : sval :: Value > (& mut self , _ : Option < sval :: Tag > , _ : sval :: Label , _ : V ,) -> Result { Err (Error :: invalid_value ("records are unsupported")) } fn end (self) -> Result < Self :: Ok > { Err (Error :: invalid_value ("records are unsupported")) } }
};
}
