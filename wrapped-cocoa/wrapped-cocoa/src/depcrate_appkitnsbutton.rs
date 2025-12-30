// Generated macro for NSButton (trait)
macro_rules! Depcrate_appkitNSButton {
() => {
// Module: crate::appkit
// Provides: {"NSButton"}
// Dependencies: {}
pub trait NSButton : Sized { unsafe fn setImage_ (self , img : id) ; unsafe fn setBezelStyle_ (self , style : NSBezelStyle) ; unsafe fn setTitle_ (self , title : id) ; unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSButton) , alloc] } unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn setTarget_ (self , target : id) ; unsafe fn setAction_ (self , selector : objc :: runtime :: Sel) ; }
};
}
