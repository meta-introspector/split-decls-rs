// Generated macro for DecompressPoint (trait)
macro_rules! Depcrate_pointDecompressPoint {
() => {
// Module: crate::point
// Provides: {"DecompressPoint"}
// Dependencies: {}
# [doc = " Decompress an elliptic curve point."] # [doc = ""] # [doc = " Point decompression recovers an original curve point from its x-coordinate"] # [doc = " and a boolean flag indicating whether or not the y-coordinate is odd."] pub trait DecompressPoint < C : Curve > : Sized { # [doc = " Attempt to decompress an elliptic curve point."] fn decompress (x : & FieldBytes < C > , y_is_odd : Choice) -> CtOption < Self > ; }
};
}
