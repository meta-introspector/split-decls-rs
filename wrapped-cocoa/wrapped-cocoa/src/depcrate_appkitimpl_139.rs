// Generated macro for impl_139 (impl)
macro_rules! Depcrate_appkitimpl_139 {
() => {
// Module: crate::appkit
// Provides: {"impl_139"}
// Dependencies: {}
impl NSMenuItem for id { unsafe fn initWithTitle_action_keyEquivalent_ (self , title : id , action : SEL , key : id) -> id { msg_send ! [self , initWithTitle : title action : action keyEquivalent : key] } unsafe fn setKeyEquivalentModifierMask_ (self , mask : NSEventModifierFlags) { msg_send ! [self , setKeyEquivalentModifierMask : mask] } unsafe fn setSubmenu_ (self , submenu : id) { msg_send ! [self , setSubmenu : submenu] } unsafe fn setTarget_ (self , target : id) { msg_send ! [self , setTarget : target] } }
};
}
