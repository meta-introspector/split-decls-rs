// Generated macro for impl_137 (impl)
macro_rules! Depcrate_appkitimpl_137 {
() => {
// Module: crate::appkit
// Provides: {"impl_137"}
// Dependencies: {}
impl NSMenu for id { unsafe fn initWithTitle_ (self , title : id) -> id { msg_send ! [self , initWithTitle : title] } unsafe fn setAutoenablesItems (self , state : BOOL) { msg_send ! [self , setAutoenablesItems : state] } unsafe fn addItem_ (self , menu_item : id) { msg_send ! [self , addItem : menu_item] } unsafe fn addItemWithTitle_action_keyEquivalent (self , title : id , action : SEL , key : id) -> id { msg_send ! [self , addItemWithTitle : title action : action keyEquivalent : key] } unsafe fn itemAtIndex_ (self , index : NSInteger) -> id { msg_send ! [self , itemAtIndex : index] } }
};
}
