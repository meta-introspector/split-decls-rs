// Generated macro for Interpolator (struct)
macro_rules! Depcrate_hyperlinkInterpolator {
() => {
// Module: crate::hyperlink
// Provides: {"Interpolator"}
// Dependencies: {}
# [doc = " An abstraction for interpolating a hyperlink format with values for every"] # [doc = " variable."] # [doc = ""] # [doc = " Interpolation of variables occurs through two different sources. The"] # [doc = " first is via a `HyperlinkEnvironment` for values that are expected to"] # [doc = " be invariant. This comes from the `HyperlinkConfig` used to build this"] # [doc = " interpolator. The second source is via `Values`, which is provided to"] # [doc = " `Interpolator::begin`. The `Values` contains things like the file path,"] # [doc = " line number and column number."] # [derive (Clone , Debug)] pub (crate) struct Interpolator { config : HyperlinkConfig , buf : RefCell < Vec < u8 > > , }
};
}
