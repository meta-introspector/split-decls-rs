// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (feature = "qlog")] impl From < UnknownTransportParameter < Vec < u8 > > > for qlog :: events :: quic :: UnknownTransportParameter { fn from (value : UnknownTransportParameter < Vec < u8 > >) -> Self { Self { id : value . id , value : qlog :: HexSlice :: maybe_string (Some (value . value . as_slice ())) . unwrap_or_default () , } } }
};
}
