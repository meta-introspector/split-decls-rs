// Generated macro for impl_204 (impl)
macro_rules! Depcrate_connect_hostimpl_204 {
() => {
// Module: crate::connect::host
// Provides: {"impl_204"}
// Dependencies: {}
impl Host for String { fn hostname (& self) -> & str { self . split_once (':') . map (| (hostname , _) | hostname) . unwrap_or (self) } fn port (& self) -> Option < u16 > { self . split_once (':') . and_then (| (_ , port) | port . parse () . ok ()) } }
};
}
