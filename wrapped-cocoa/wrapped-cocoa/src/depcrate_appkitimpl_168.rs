// Generated macro for impl_168 (impl)
macro_rules! Depcrate_appkitimpl_168 {
() => {
// Module: crate::appkit
// Provides: {"impl_168"}
// Dependencies: {}
impl NSOpenGLContext for id { unsafe fn initWithFormat_shareContext_ (self , format : id , shareContext : id ,) -> id { msg_send ! [self , initWithFormat : format shareContext : shareContext] } unsafe fn initWithCGLContextObj_ (self , context : CGLContextObj) -> id { msg_send ! [self , initWithCGLContextObj : context] } unsafe fn clearCurrentContext (_ : Self) { msg_send ! [class ! (NSOpenGLContext) , clearCurrentContext] } unsafe fn currentContext (_ : Self) -> id { msg_send ! [class ! (NSOpenGLContext) , currentContext] } unsafe fn makeCurrentContext (self) { msg_send ! [self , makeCurrentContext] } unsafe fn setView_ (self , view : id) { msg_send ! [self , setView : view] } unsafe fn view (self) -> id { msg_send ! [self , view] } unsafe fn clearDrawable (self) { msg_send ! [self , clearDrawable] } unsafe fn update (self) { msg_send ! [self , update] } unsafe fn flushBuffer (self) { msg_send ! [self , flushBuffer] } unsafe fn setValues_forParameter_ (self , vals : * const GLint , param : NSOpenGLContextParameter) { msg_send ! [self , setValues : vals forParameter : param] } unsafe fn getValues_forParameter_ (self , vals : * mut GLint , param : NSOpenGLContextParameter) { msg_send ! [self , getValues : vals forParameter : param] } unsafe fn setCurrentVirtualScreen_ (self , screen : GLint) { msg_send ! [self , setCurrentVirtualScreen : screen] } unsafe fn currentVirtualScreen (self) -> GLint { msg_send ! [self , currentVirtualScreen] } unsafe fn CGLContextObj (self) -> CGLContextObj { msg_send ! [self , CGLContextObj] } }
};
}
