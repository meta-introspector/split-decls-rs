// Generated macro for impl_28 (impl)
macro_rules! Depcrate_errorimpl_28 {
() => {
// Module: crate::error
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] impl From < Error < & [u8] > > for Error < crate :: lib :: std :: vec :: Vec < u8 > > { fn from (value : Error < & [u8] >) -> Self { Error { input : value . input . to_owned () , code : value . code , } } }
};
}
