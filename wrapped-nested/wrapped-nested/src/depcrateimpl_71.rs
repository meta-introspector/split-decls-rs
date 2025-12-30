// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'sval , Ok > StreamTuple < 'sval > for Unsupported < Ok > { type Ok = Ok ; fn value_computed < V : sval :: Value > (& mut self , _ : Option < sval :: Tag > , _ : sval :: Index , _ : V ,) -> Result { Err (Error :: invalid_value ("tuples are unsupported")) } fn end (self) -> Result < Self :: Ok > { Err (Error :: invalid_value ("tuples are unsupported")) } }
};
}
