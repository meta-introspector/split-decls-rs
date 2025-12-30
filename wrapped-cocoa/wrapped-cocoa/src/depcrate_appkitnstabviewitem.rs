// Generated macro for NSTabViewItem (trait)
macro_rules! Depcrate_appkitNSTabViewItem {
() => {
// Module: crate::appkit
// Provides: {"NSTabViewItem"}
// Dependencies: {}
pub trait NSTabViewItem : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSTabViewItem) , alloc] } unsafe fn new (_ : Self) -> id { msg_send ! [class ! (NSTabViewItem) , new] } unsafe fn initWithIdentifier_ (self , identifier : id) -> id ; unsafe fn drawLabel_inRect_ (self , shouldTruncateLabel : BOOL , labelRect : NSRect) ; unsafe fn label (self) -> id ; unsafe fn setLabel_ (self , label : id) ; unsafe fn sizeOfLabel_ (self , computeMin : BOOL) ; unsafe fn tabState (self) -> NSTabState ; unsafe fn identifier (self) -> id ; unsafe fn setIdentifier_ (self , identifier : id) ; unsafe fn color (self) -> id ; unsafe fn setColor_ (self , color : id) ; unsafe fn view (self) -> id ; unsafe fn setView_ (self , view : id) ; unsafe fn initialFirstResponder (self) -> id ; unsafe fn setInitialFirstResponder_ (self , initialFirstResponder : id) ; unsafe fn tabView (self) -> id ; unsafe fn tooltip (self) -> id ; unsafe fn setToolTip_ (self , toolTip : id) ; }
};
}
