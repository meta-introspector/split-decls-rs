// Generated macro for NSToolbar (trait)
macro_rules! Depcrate_appkitNSToolbar {
() => {
// Module: crate::appkit
// Provides: {"NSToolbar"}
// Dependencies: {}
pub trait NSToolbar : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSToolbar) , alloc] } unsafe fn init_ (self) -> id ; unsafe fn initWithIdentifier_ (self , identifier : id) -> id ; unsafe fn showsBaselineSeparator (self) -> BOOL ; unsafe fn setShowsBaselineSeparator_ (self , value : BOOL) ; }
};
}
