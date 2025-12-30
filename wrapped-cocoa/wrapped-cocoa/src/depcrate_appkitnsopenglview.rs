// Generated macro for NSOpenGLView (trait)
macro_rules! Depcrate_appkitNSOpenGLView {
() => {
// Module: crate::appkit
// Provides: {"NSOpenGLView"}
// Dependencies: {}
pub trait NSOpenGLView : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSOpenGLView) , alloc] } unsafe fn initWithFrame_pixelFormat_ (self , frameRect : NSRect , format : id) -> id ; unsafe fn display_ (self) ; unsafe fn setOpenGLContext_ (self , context : id) ; unsafe fn setPixelFormat_ (self , pixelformat : id) ; }
};
}
