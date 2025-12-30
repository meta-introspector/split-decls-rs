// Generated macro for DecompactPoint (trait)
macro_rules! Depcrate_pointDecompactPoint {
() => {
// Module: crate::point
// Provides: {"DecompactPoint"}
// Dependencies: {}
# [doc = " Decompact an elliptic curve point from an x-coordinate."] # [doc = ""] # [doc = " Decompaction relies on properties of specially-generated keys but provides"] # [doc = " a more compact representation than standard point compression."] pub trait DecompactPoint < C : Curve > : Sized { # [doc = " Attempt to decompact an elliptic curve point"] fn decompact (x : & FieldBytes < C >) -> CtOption < Self > ; }
};
}
