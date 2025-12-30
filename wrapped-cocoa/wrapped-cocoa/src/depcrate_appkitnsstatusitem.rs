// Generated macro for NSStatusItem (trait)
macro_rules! Depcrate_appkitNSStatusItem {
() => {
// Module: crate::appkit
// Provides: {"NSStatusItem"}
// Dependencies: {}
pub trait NSStatusItem : Sized { unsafe fn statusBar (self) -> id ; unsafe fn button (self) -> id ; unsafe fn menu (self) -> id ; unsafe fn setMenu_ (self , menu : id) ; unsafe fn length (self) -> CGFloat ; unsafe fn setLength_ (self , length : CGFloat) ; }
};
}
