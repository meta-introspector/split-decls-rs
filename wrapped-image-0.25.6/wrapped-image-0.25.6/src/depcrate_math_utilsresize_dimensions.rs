// Generated macro for resize_dimensions (function)
macro_rules! Depcrate_math_utilsresize_dimensions {
() => {
// Module: crate::math::utils
// Provides: {"resize_dimensions"}
// Dependencies: {}
# [doc = " Calculates the width and height an image should be resized to."] # [doc = " This preserves aspect ratio, and based on the `fill` parameter"] # [doc = " will either fill the dimensions to fit inside the smaller constraint"] # [doc = " (will overflow the specified bounds on one axis to preserve"] # [doc = " aspect ratio), or will shrink so that both dimensions are"] # [doc = " completely contained within the given `width` and `height`,"] # [doc = " with empty space on one axis."] pub (crate) fn resize_dimensions (width : u32 , height : u32 , nwidth : u32 , nheight : u32 , fill : bool ,) -> (u32 , u32) { let wratio = f64 :: from (nwidth) / f64 :: from (width) ; let hratio = f64 :: from (nheight) / f64 :: from (height) ; let ratio = if fill { f64 :: max (wratio , hratio) } else { f64 :: min (wratio , hratio) } ; let nw = max ((f64 :: from (width) * ratio) . round () as u64 , 1) ; let nh = max ((f64 :: from (height) * ratio) . round () as u64 , 1) ; if nw > u64 :: from (u32 :: MAX) { let ratio = f64 :: from (u32 :: MAX) / f64 :: from (width) ; (u32 :: MAX , max ((f64 :: from (height) * ratio) . round () as u32 , 1)) } else if nh > u64 :: from (u32 :: MAX) { let ratio = f64 :: from (u32 :: MAX) / f64 :: from (height) ; (max ((f64 :: from (width) * ratio) . round () as u32 , 1) , u32 :: MAX) } else { (nw as u32 , nh as u32) } }
};
}
