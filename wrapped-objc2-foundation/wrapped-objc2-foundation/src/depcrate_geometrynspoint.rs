// Generated macro for NSPoint (type)
macro_rules! Depcrate_geometryNSPoint {
() => {
// Module: crate::geometry
// Provides: {"NSPoint"}
// Dependencies: {}
# [doc = " A point in a Cartesian coordinate system."] # [doc = ""] # [doc = " This is a convenience alias for [`CGPoint`]. For ease of use, it is"] # [doc = " available on all platforms, though in practice it is only useful on macOS."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/foundation/nspoint?language=objc)."] # [cfg (feature = "objc2-core-foundation")] pub type NSPoint = CGPoint ;
};
}
