// Generated macro for impl_29 (impl)
macro_rules! Depcrate_errorimpl_29 {
() => {
// Module: crate::error
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] impl From < Error < & str > > for Error < crate :: lib :: std :: string :: String > { fn from (value : Error < & str >) -> Self { Error { input : value . input . to_owned () , code : value . code , } } }
};
}
