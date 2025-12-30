// Generated macro for CGRect (struct)
macro_rules! Depcrate_geometryCGRect {
() => {
// Module: crate::geometry
// Provides: {"CGRect"}
// Dependencies: {}
# [doc = " The location and dimensions of a rectangle."] # [doc = ""] # [doc = " In the default Core Graphics coordinate space (macOS), the origin is"] # [doc = " located in the lower-left corner of the rectangle and the rectangle"] # [doc = " extends towards the upper-right corner."] # [doc = ""] # [doc = " If the context has a flipped coordinate space (iOS, tvOS, watchOS) the"] # [doc = " origin is in the upper-left corner and the rectangle extends towards the"] # [doc = " lower-right corner."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgrect?language=objc)."] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct CGRect { # [doc = " The coordinates of the rectangle’s origin."] pub origin : CGPoint , # [doc = " The dimensions of the rectangle."] pub size : CGSize , }
};
}
