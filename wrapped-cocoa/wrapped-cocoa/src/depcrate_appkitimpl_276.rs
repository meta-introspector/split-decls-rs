// Generated macro for impl_276 (impl)
macro_rules! Depcrate_appkitimpl_276 {
() => {
// Module: crate::appkit
// Provides: {"impl_276"}
// Dependencies: {}
impl NSStatusBar for id { unsafe fn statusItemWithLength_ (self , length : CGFloat) -> id { msg_send ! [self , statusItemWithLength : length] } unsafe fn removeStatusItem_ (self , item : id) { msg_send ! [self , removeStatusItem : item] } unsafe fn isVertical (self) -> BOOL { msg_send ! [self , isVertical] } }
};
}
