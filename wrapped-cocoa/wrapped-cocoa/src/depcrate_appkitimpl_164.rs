// Generated macro for impl_164 (impl)
macro_rules! Depcrate_appkitimpl_164 {
() => {
// Module: crate::appkit
// Provides: {"impl_164"}
// Dependencies: {}
impl NSOpenGLView for id { unsafe fn initWithFrame_pixelFormat_ (self , frameRect : NSRect , format : id) -> id { msg_send ! [self , initWithFrame : frameRect pixelFormat : format] } unsafe fn display_ (self) { msg_send ! [self , display] } unsafe fn setOpenGLContext_ (self , context : id) { msg_send ! [self , setOpenGLContext : context] } unsafe fn setPixelFormat_ (self , pixelformat : id) { msg_send ! [self , setPixelFormat : pixelformat] } }
};
}
