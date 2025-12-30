// Generated macro for HyperlinkConfig (struct)
macro_rules! Depcrate_hyperlinkHyperlinkConfig {
() => {
// Module: crate::hyperlink
// Provides: {"HyperlinkConfig"}
// Dependencies: {}
# [doc = " Hyperlink configuration."] # [doc = ""] # [doc = " This configuration specifies both the [hyperlink format](HyperlinkFormat)"] # [doc = " and an [environment](HyperlinkConfig) for interpolating a subset of"] # [doc = " variables. The specific subset includes variables that are intended to"] # [doc = " be invariant throughout the lifetime of a process, such as a machine's"] # [doc = " hostname."] # [doc = ""] # [doc = " A hyperlink configuration can be provided to printer builders such as"] # [doc = " [`StandardBuilder::hyperlink`](crate::StandardBuilder::hyperlink)."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct HyperlinkConfig (Arc < HyperlinkConfigInner >) ;
};
}
