// Generated macro for NSStatusBar (trait)
macro_rules! Depcrate_appkitNSStatusBar {
() => {
// Module: crate::appkit
// Provides: {"NSStatusBar"}
// Dependencies: {}
pub trait NSStatusBar : Sized { unsafe fn systemStatusBar (_ : Self) -> id { msg_send ! [class ! (NSStatusBar) , systemStatusBar] } unsafe fn statusItemWithLength_ (self , length : CGFloat) -> id ; unsafe fn removeStatusItem_ (self , item : id) ; unsafe fn isVertical (self) -> BOOL ; }
};
}
