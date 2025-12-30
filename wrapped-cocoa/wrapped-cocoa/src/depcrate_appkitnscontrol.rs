// Generated macro for NSControl (trait)
macro_rules! Depcrate_appkitNSControl {
() => {
// Module: crate::appkit
// Provides: {"NSControl"}
// Dependencies: {}
pub trait NSControl : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSControl) , alloc] } unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn isEnabled_ (self) -> BOOL ; unsafe fn setEnabled_ (self , enabled : BOOL) -> BOOL ; }
};
}
