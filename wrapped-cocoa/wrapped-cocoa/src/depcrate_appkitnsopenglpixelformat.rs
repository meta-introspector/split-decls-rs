// Generated macro for NSOpenGLPixelFormat (trait)
macro_rules! Depcrate_appkitNSOpenGLPixelFormat {
() => {
// Module: crate::appkit
// Provides: {"NSOpenGLPixelFormat"}
// Dependencies: {}
pub trait NSOpenGLPixelFormat : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSOpenGLPixelFormat) , alloc] } unsafe fn initWithAttributes_ (self , attributes : & [u32]) -> id ; unsafe fn getValues_forAttribute_forVirtualScreen_ (self , val : * mut GLint , attrib : NSOpenGLPixelFormatAttribute , screen : GLint ,) ; unsafe fn numberOfVirtualScreens (self) -> GLint ; }
};
}
