// Generated macro for ArcSwap (type)
macro_rules! DepcrateArcSwap {
() => {
// Module: crate
// Provides: {"ArcSwap"}
// Dependencies: {}
# [doc = " An atomic storage for `Arc`."] # [doc = ""] # [doc = " This is a type alias only. Most of its methods are described on"] # [doc = " [`ArcSwapAny`](struct.ArcSwapAny.html)."] pub type ArcSwap < T > = ArcSwapAny < Arc < T > > ;
};
}
