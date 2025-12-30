// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "http")] impl HeaderValue for http :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
};
}
