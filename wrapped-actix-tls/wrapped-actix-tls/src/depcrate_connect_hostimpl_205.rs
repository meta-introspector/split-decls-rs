// Generated macro for impl_205 (impl)
macro_rules! Depcrate_connect_hostimpl_205 {
() => {
// Module: crate::connect::host
// Provides: {"impl_205"}
// Dependencies: {}
impl Host for & 'static str { fn hostname (& self) -> & str { self . split_once (':') . map (| (hostname , _) | hostname) . unwrap_or (self) } fn port (& self) -> Option < u16 > { self . split_once (':') . and_then (| (_ , port) | port . parse () . ok ()) } }
};
}
