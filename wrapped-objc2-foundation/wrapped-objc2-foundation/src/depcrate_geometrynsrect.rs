// Generated macro for NSRect (type)
macro_rules! Depcrate_geometryNSRect {
() => {
// Module: crate::geometry
// Provides: {"NSRect"}
// Dependencies: {}
# [doc = " A rectangle."] # [doc = ""] # [doc = " This is a convenience alias for [`CGRect`]. For ease of use, it is"] # [doc = " available on all platforms, though in practice it is only useful on macOS."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrect?language=objc)."] # [cfg (feature = "objc2-core-foundation")] pub type NSRect = CGRect ;
};
}
