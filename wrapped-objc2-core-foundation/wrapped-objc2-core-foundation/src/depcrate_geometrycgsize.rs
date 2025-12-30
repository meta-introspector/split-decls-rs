// Generated macro for CGSize (struct)
macro_rules! Depcrate_geometryCGSize {
() => {
// Module: crate::geometry
// Provides: {"CGSize"}
// Dependencies: {}
# [doc = " A two-dimensional size."] # [doc = ""] # [doc = " As this is sometimes used to represent a distance vector, rather than a"] # [doc = " physical size, the width and height are _not_ guaranteed to be"] # [doc = " non-negative! Methods that expect that must use one of [`CGSize::abs`] or"] # [doc = " [`CGRect::standardize`]."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgsize?language=objc)."] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct CGSize { # [doc = " The dimensions along the x-axis."] pub width : CGFloat , # [doc = " The dimensions along the y-axis."] pub height : CGFloat , }
};
}
