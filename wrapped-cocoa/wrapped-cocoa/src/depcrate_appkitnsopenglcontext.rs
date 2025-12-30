// Generated macro for NSOpenGLContext (trait)
macro_rules! Depcrate_appkitNSOpenGLContext {
() => {
// Module: crate::appkit
// Provides: {"NSOpenGLContext"}
// Dependencies: {}
pub trait NSOpenGLContext : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSOpenGLContext) , alloc] } unsafe fn initWithFormat_shareContext_ (self , format : id , shareContext : id ,) -> id ; unsafe fn initWithCGLContextObj_ (self , context : CGLContextObj) -> id ; unsafe fn clearCurrentContext (_ : Self) ; unsafe fn currentContext (_ : Self) -> id ; unsafe fn makeCurrentContext (self) ; unsafe fn setView_ (self , view : id) ; unsafe fn view (self) -> id ; unsafe fn clearDrawable (self) ; unsafe fn update (self) ; unsafe fn flushBuffer (self) ; unsafe fn setValues_forParameter_ (self , vals : * const GLint , param : NSOpenGLContextParameter) ; unsafe fn getValues_forParameter_ (self , vals : * mut GLint , param : NSOpenGLContextParameter) ; unsafe fn setCurrentVirtualScreen_ (self , screen : GLint) ; unsafe fn currentVirtualScreen (self) -> GLint ; unsafe fn CGLContextObj (self) -> CGLContextObj ; }
};
}
