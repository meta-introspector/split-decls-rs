// Generated macro for impl_1705 (impl)
macro_rules! Depcrate_geometryimpl_1705 {
() => {
// Module: crate::geometry
// Provides: {"impl_1705"}
// Dependencies: {}
impl CGPoint { # [doc = " Create a new point with the given coordinates."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_core_foundation::CGPoint;"] # [doc = " assert_eq!(CGPoint::new(10.0, -2.3), CGPoint { x: 10.0, y: -2.3 });"] # [doc = " ```"] # [inline] # [doc (alias = "NSMakePoint")] # [doc (alias = "CGPointMake")] pub const fn new (x : CGFloat , y : CGFloat) -> Self { Self { x , y } } # [doc = " A point with both coordinates set to `0.0`."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use objc2_core_foundation::CGPoint;"] # [doc = " assert_eq!(CGPoint::ZERO, CGPoint { x: 0.0, y: 0.0 });"] # [doc = " ```"] # [doc (alias = "NSZeroPoint")] # [doc (alias = "CGPointZero")] # [doc (alias = "ORIGIN")] pub const ZERO : Self = Self :: new (0.0 , 0.0) ; }
};
}
