// Generated macro for impl_61 (impl)
macro_rules! Depcrate_foundationimpl_61 {
() => {
// Module: crate::foundation
// Provides: {"impl_61"}
// Dependencies: {}
impl NSBundle for id { unsafe fn mainBundle () -> id { msg_send ! [class ! (NSBundle) , mainBundle] } unsafe fn loadNibNamed_owner_topLevelObjects_ (self , name : id , owner : id , topLevelObjects : * mut id ,) -> BOOL { msg_send ! [self , loadNibNamed : name owner : owner topLevelObjects : topLevelObjects] } unsafe fn bundleIdentifier (self) -> id { msg_send ! [self , bundleIdentifier] } unsafe fn resourcePath (self) -> id { msg_send ! [self , resourcePath] } }
};
}
