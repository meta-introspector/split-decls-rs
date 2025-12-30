// Generated macro for impl_433 (impl)
macro_rules! Depcrate_data_valueimpl_433 {
() => {
// Module: crate::data_value
// Provides: {"impl_433"}
// Dependencies: {}
impl Display for DataValueCastFailure { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { DataValueCastFailure :: TryInto (from , to) => { write ! (f , "unable to cast data value of type {from} to type {to}") } DataValueCastFailure :: FromInteger (val , to) => { write ! (f , "unable to cast i64({val}) to a data value of type {to}") } } } }
};
}
