// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl From < UnknownTransportParameter < & [u8] > > for UnknownTransportParameter < Vec < u8 > > { fn from (value : UnknownTransportParameter < & [u8] >) -> Self { Self { id : value . id , value : value . value . to_vec () , } } }
};
}
