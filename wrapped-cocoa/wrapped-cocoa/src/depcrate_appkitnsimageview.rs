// Generated macro for NSImageView (trait)
macro_rules! Depcrate_appkitNSImageView {
() => {
// Module: crate::appkit
// Provides: {"NSImageView"}
// Dependencies: {}
pub trait NSImageView : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSImageView) , alloc] } unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn setImage_ (self , img : id) ; }
};
}
