// Generated macro for other_256 (other)
macro_rules! Depcrate_imageother_256 {
() => {
// Module: crate::image
// Provides: {"other_256"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGImageGetTypeID () -> CFTypeID ; fn CGImageGetWidth (image : crate :: sys :: CGImageRef) -> usize ; fn CGImageGetHeight (image : crate :: sys :: CGImageRef) -> usize ; fn CGImageGetBitsPerComponent (image : crate :: sys :: CGImageRef) -> usize ; fn CGImageGetBitsPerPixel (image : crate :: sys :: CGImageRef) -> usize ; fn CGImageGetBytesPerRow (image : crate :: sys :: CGImageRef) -> usize ; fn CGImageGetColorSpace (image : crate :: sys :: CGImageRef) -> crate :: sys :: CGColorSpaceRef ; fn CGImageGetDataProvider (image : crate :: sys :: CGImageRef) -> crate :: sys :: CGDataProviderRef ; fn CGImageRelease (image : crate :: sys :: CGImageRef) ; fn CGImageCreate (width : usize , height : usize , bitsPerComponent : usize , bitsPerPixel : usize , bytesPerRow : usize , space : crate :: sys :: CGColorSpaceRef , bitmapInfo : u32 , provider : crate :: sys :: CGDataProviderRef , decode : * const CGFloat , shouldInterpolate : bool , intent : u32 ,) -> crate :: sys :: CGImageRef ; fn CGImageCreateWithImageInRect (image : crate :: sys :: CGImageRef , rect : CGRect ,) -> crate :: sys :: CGImageRef ; }
};
}
