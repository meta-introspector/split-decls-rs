// Generated macro for NSTextField (trait)
macro_rules! Depcrate_appkitNSTextField {
() => {
// Module: crate::appkit
// Provides: {"NSTextField"}
// Dependencies: {}
pub trait NSTextField : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSTextField) , alloc] } unsafe fn initWithFrame_ (self , frameRect : NSRect) -> id ; unsafe fn setEditable_ (self , editable : BOOL) ; unsafe fn setStringValue_ (self , label : id) ; }
};
}
