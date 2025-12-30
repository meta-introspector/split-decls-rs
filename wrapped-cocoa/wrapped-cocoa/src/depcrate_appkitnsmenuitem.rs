// Generated macro for NSMenuItem (trait)
macro_rules! Depcrate_appkitNSMenuItem {
() => {
// Module: crate::appkit
// Provides: {"NSMenuItem"}
// Dependencies: {}
pub trait NSMenuItem : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSMenuItem) , alloc] } unsafe fn new (_ : Self) -> id { msg_send ! [class ! (NSMenuItem) , new] } unsafe fn separatorItem (_ : Self) -> id { msg_send ! [class ! (NSMenuItem) , separatorItem] } unsafe fn initWithTitle_action_keyEquivalent_ (self , title : id , action : SEL , key : id) -> id ; unsafe fn setKeyEquivalentModifierMask_ (self , mask : NSEventModifierFlags) ; unsafe fn setSubmenu_ (self , submenu : id) ; unsafe fn setTarget_ (self , target : id) ; }
};
}
