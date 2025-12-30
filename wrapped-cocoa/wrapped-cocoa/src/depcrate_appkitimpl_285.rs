// Generated macro for impl_285 (impl)
macro_rules! Depcrate_appkitimpl_285 {
() => {
// Module: crate::appkit
// Provides: {"impl_285"}
// Dependencies: {}
impl NSTabViewItem for id { unsafe fn initWithIdentifier_ (self , identifier : id) -> id { msg_send ! [self , initWithIdentifier : identifier] } unsafe fn drawLabel_inRect_ (self , shouldTruncateLabel : BOOL , labelRect : NSRect) { msg_send ! [self , drawLabel : shouldTruncateLabel as c_int inRect : labelRect] } unsafe fn label (self) -> id { msg_send ! [self , label] } unsafe fn setLabel_ (self , label : id) { msg_send ! [self , setLabel : label] } unsafe fn sizeOfLabel_ (self , computeMin : BOOL) { msg_send ! [self , sizeOfLabel : computeMin as c_int] } unsafe fn tabState (self) -> NSTabState { msg_send ! [self , tabState] } unsafe fn identifier (self) -> id { msg_send ! [self , identifier] } unsafe fn setIdentifier_ (self , identifier : id) { msg_send ! [self , identifier : identifier] } unsafe fn color (self) -> id { msg_send ! [self , color] } unsafe fn setColor_ (self , color : id) { msg_send ! [self , color : color] } unsafe fn view (self) -> id { msg_send ! [self , view] } unsafe fn setView_ (self , view : id) { msg_send ! [self , setView : view] } unsafe fn initialFirstResponder (self) -> id { msg_send ! [self , initialFirstResponder] } unsafe fn setInitialFirstResponder_ (self , initialFirstResponder : id) { msg_send ! [self , setInitialFirstResponder : initialFirstResponder] } unsafe fn tabView (self) -> id { msg_send ! [self , tabView] } unsafe fn tooltip (self) -> id { msg_send ! [self , tooltip] } unsafe fn setToolTip_ (self , toolTip : id) { msg_send ! [self , setToolTip : toolTip] } }
};
}
