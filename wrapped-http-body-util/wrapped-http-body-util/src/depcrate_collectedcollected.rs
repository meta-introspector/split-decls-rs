// Generated macro for Collected (struct)
macro_rules! Depcrate_collectedCollected {
() => {
// Module: crate::collected
// Provides: {"Collected"}
// Dependencies: {}
# [doc = " A collected body produced by [`BodyExt::collect`] which collects all the DATA frames"] # [doc = " and trailers."] # [doc = ""] # [doc = " [`BodyExt::collect`]: crate::BodyExt::collect"] # [derive (Debug)] pub struct Collected < B > { bufs : BufList < B > , trailers : Option < HeaderMap > , }
};
}
