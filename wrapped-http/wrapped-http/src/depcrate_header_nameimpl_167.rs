// Generated macro for impl_167 (impl)
macro_rules! Depcrate_header_nameimpl_167 {
() => {
// Module: crate::header::name
// Provides: {"impl_167"}
// Dependencies: {}
# [doc (hidden)] impl < T > From < Repr < T > > for Bytes where T : Into < Bytes > , { fn from (repr : Repr < T >) -> Bytes { match repr { Repr :: Standard (header) => Bytes :: from_static (header . as_str () . as_bytes ()) , Repr :: Custom (header) => header . into () , } } }
};
}
