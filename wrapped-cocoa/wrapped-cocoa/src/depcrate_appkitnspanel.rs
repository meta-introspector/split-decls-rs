// Generated macro for NSPanel (trait)
macro_rules! Depcrate_appkitNSPanel {
() => {
// Module: crate::appkit
// Provides: {"NSPanel"}
// Dependencies: {}
pub trait NSPanel : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSPanel) , alloc] } unsafe fn setBecomesKeyOnlyIfNeeded (self , becomesKeyOnlyIfNeeded : BOOL) ; unsafe fn becomesKeyOnlyIfNeeded (self) -> BOOL ; unsafe fn setFloatingPanel (self , floatingPanel : BOOL) ; unsafe fn floatingPanel (self) -> BOOL ; unsafe fn setWorksWhenModal (self , worksWithPanel : BOOL) ; }
};
}
