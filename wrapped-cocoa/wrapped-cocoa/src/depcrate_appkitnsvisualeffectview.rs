// Generated macro for NSVisualEffectView (trait)
macro_rules! Depcrate_appkitNSVisualEffectView {
() => {
// Module: crate::appkit
// Provides: {"NSVisualEffectView"}
// Dependencies: {}
# [allow (non_snake_case)] pub trait NSVisualEffectView : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSVisualEffectView) , alloc] } unsafe fn init (self) -> id ; unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn bounds (self) -> NSRect ; unsafe fn frame (self) -> NSRect ; unsafe fn setFrameSize (self , frameSize : NSSize) ; unsafe fn setFrameOrigin (self , frameOrigin : NSPoint) ; unsafe fn superview (self) -> id ; unsafe fn removeFromSuperview (self) ; unsafe fn isEmphasized (self) -> BOOL ; unsafe fn setEmphasized_ (self , emphasized : BOOL) ; unsafe fn setMaterial_ (self , material : NSVisualEffectMaterial) ; unsafe fn setState_ (self , state : NSVisualEffectState) ; unsafe fn setBlendingMode_ (self , mode : NSVisualEffectBlendingMode) ; }
};
}
