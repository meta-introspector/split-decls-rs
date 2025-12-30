// Generated macro for impl_274 (impl)
macro_rules! Depcrate_appkitimpl_274 {
() => {
// Module: crate::appkit
// Provides: {"impl_274"}
// Dependencies: {}
impl NSStatusItem for id { unsafe fn statusBar (self) -> id { msg_send ! [self , statusBar] } unsafe fn button (self) -> id { msg_send ! [self , button] } unsafe fn menu (self) -> id { msg_send ! [self , menu] } unsafe fn setMenu_ (self , menu : id) { msg_send ! [self , setMenu : menu] } unsafe fn length (self) -> CGFloat { msg_send ! [self , length] } unsafe fn setLength_ (self , length : CGFloat) { msg_send ! [self , setLength : length] } }
};
}
