// Generated macro for NSView (trait)
macro_rules! Depcrate_appkitNSView {
() => {
// Module: crate::appkit
// Provides: {"NSView"}
// Dependencies: {}
pub trait NSView : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSView) , alloc] } unsafe fn init (self) -> id ; unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn bounds (self) -> NSRect ; unsafe fn frame (self) -> NSRect ; unsafe fn setFrameSize (self , frameSize : NSSize) ; unsafe fn setFrameOrigin (self , frameOrigin : NSPoint) ; unsafe fn display_ (self) ; unsafe fn setWantsBestResolutionOpenGLSurface_ (self , flag : BOOL) ; unsafe fn convertPoint_fromView_ (self , point : NSPoint , view : id) -> NSPoint ; unsafe fn addSubview_ (self , view : id) ; unsafe fn superview (self) -> id ; unsafe fn removeFromSuperview (self) ; unsafe fn setAutoresizingMask_ (self , autoresizingMask : NSAutoresizingMaskOptions) ; unsafe fn wantsLayer (self) -> BOOL ; unsafe fn setWantsLayer (self , wantsLayer : BOOL) ; unsafe fn layer (self) -> id ; unsafe fn setLayer (self , layer : id) ; unsafe fn widthAnchor (self) -> id ; unsafe fn heightAnchor (self) -> id ; unsafe fn convertRectToBacking (self , rect : NSRect) -> NSRect ; unsafe fn layerContentsPlacement (self) -> NSViewLayerContentsPlacement ; unsafe fn setLayerContentsPlacement (self , placement : NSViewLayerContentsPlacement) ; unsafe fn setAppearance (self , appearance : id) ; }
};
}
