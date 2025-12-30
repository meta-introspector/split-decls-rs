// Generated macro for Values (struct)
macro_rules! Depcrate_hyperlinkValues {
() => {
// Module: crate::hyperlink
// Provides: {"Values"}
// Dependencies: {}
# [doc = " The values to replace the format variables with."] # [doc = ""] # [doc = " This only consists of values that depend on each path or match printed."] # [doc = " Values that are invariant throughout the lifetime of the process are set"] # [doc = " via a [`HyperlinkEnvironment`]."] # [derive (Clone , Debug)] pub (crate) struct Values < 'a > { path : & 'a HyperlinkPath , line : Option < u64 > , column : Option < u64 > , }
};
}
