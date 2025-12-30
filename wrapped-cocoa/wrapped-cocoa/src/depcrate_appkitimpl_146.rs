// Generated macro for impl_146 (impl)
macro_rules! Depcrate_appkitimpl_146 {
() => {
// Module: crate::appkit
// Provides: {"impl_146"}
// Dependencies: {}
impl NSPanel for id { unsafe fn setBecomesKeyOnlyIfNeeded (self , becomesKeyOnlyIfNeeded : BOOL) { msg_send ! [self , setBecomesKeyOnlyIfNeeded : becomesKeyOnlyIfNeeded] } unsafe fn becomesKeyOnlyIfNeeded (self) -> BOOL { msg_send ! [self , becomesKeyOnlyIfNeeded] } unsafe fn setFloatingPanel (self , floatingPanel : BOOL) { msg_send ! [self , setFloatingPanel : floatingPanel] } unsafe fn floatingPanel (self) -> BOOL { msg_send ! [self , isFloatingPanel] } unsafe fn setWorksWhenModal (self , worksWhenModal : BOOL) { msg_send ! [self , setWorksWhenModal : worksWhenModal] } }
};
}
