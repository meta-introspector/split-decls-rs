// Generated macro for impl_205 (impl)
macro_rules! Depcrate_encodeimpl_205 {
() => {
// Module: crate::encode
// Provides: {"impl_205"}
// Dependencies: {}
impl < E : RmpWriteErr > From < DataWriteError < E > > for ValueWriteError < E > { # [cold] fn from (err : DataWriteError < E >) -> Self { match err { DataWriteError (err) => Self :: InvalidDataWrite (err) , } } }
};
}
