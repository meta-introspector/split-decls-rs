// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "http10")] impl HeaderValue for http10 :: HeaderValue { fn to_str (& self) -> Result < & str , ToStrError > { self . to_str () . map_err (| _ | ToStrError { _priv : () }) } }
};
}
