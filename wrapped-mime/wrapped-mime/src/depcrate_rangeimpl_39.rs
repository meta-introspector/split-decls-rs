// Generated macro for impl_39 (impl)
macro_rules! Depcrate_rangeimpl_39 {
() => {
// Module: crate::range
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " Any `MediaType` can freely be a `MediaRange`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // If we only supported `text/plain`:"] # [doc = " let range = mime::MediaRange::from(mime::TEXT_PLAIN);"] # [doc = " ```"] impl From < MediaType > for MediaRange { fn from (mt : MediaType) -> MediaRange { MediaRange { mime : mt . mime , } } }
};
}
