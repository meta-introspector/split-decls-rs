// Generated macro for impl_166 (impl)
macro_rules! Depcrate_appkitimpl_166 {
() => {
// Module: crate::appkit
// Provides: {"impl_166"}
// Dependencies: {}
impl NSOpenGLPixelFormat for id { unsafe fn initWithAttributes_ (self , attributes : & [u32]) -> id { msg_send ! [self , initWithAttributes : attributes . as_ptr ()] } unsafe fn getValues_forAttribute_forVirtualScreen_ (self , val : * mut GLint , attrib : NSOpenGLPixelFormatAttribute , screen : GLint ,) { msg_send ! [self , getValues : val forAttribute : attrib forVirtualScreen : screen] } unsafe fn numberOfVirtualScreens (self) -> GLint { msg_send ! [self , numberOfVirtualScreens] } }
};
}
