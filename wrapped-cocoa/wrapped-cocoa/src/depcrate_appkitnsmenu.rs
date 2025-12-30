// Generated macro for NSMenu (trait)
macro_rules! Depcrate_appkitNSMenu {
() => {
// Module: crate::appkit
// Provides: {"NSMenu"}
// Dependencies: {}
pub trait NSMenu : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSMenu) , alloc] } unsafe fn new (_ : Self) -> id { msg_send ! [class ! (NSMenu) , new] } unsafe fn initWithTitle_ (self , title : id) -> id ; unsafe fn setAutoenablesItems (self , state : BOOL) ; unsafe fn addItem_ (self , menu_item : id) ; unsafe fn addItemWithTitle_action_keyEquivalent (self , title : id , action : SEL , key : id) -> id ; unsafe fn itemAtIndex_ (self , index : NSInteger) -> id ; }
};
}
