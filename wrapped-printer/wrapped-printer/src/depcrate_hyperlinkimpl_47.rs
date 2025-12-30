// Generated macro for impl_47 (impl)
macro_rules! Depcrate_hyperlinkimpl_47 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_47"}
// Dependencies: {}
impl HyperlinkConfig { # [doc = " Create a new configuration from an environment and a format."] pub fn new (env : HyperlinkEnvironment , format : HyperlinkFormat ,) -> HyperlinkConfig { HyperlinkConfig (Arc :: new (HyperlinkConfigInner { env , format })) } # [doc = " Returns the hyperlink environment in this configuration."] pub (crate) fn environment (& self) -> & HyperlinkEnvironment { & self . 0 . env } # [doc = " Returns the hyperlink format in this configuration."] pub (crate) fn format (& self) -> & HyperlinkFormat { & self . 0 . format } }
};
}
