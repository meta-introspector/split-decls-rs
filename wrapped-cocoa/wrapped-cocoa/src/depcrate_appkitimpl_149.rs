// Generated macro for impl_149 (impl)
macro_rules! Depcrate_appkitimpl_149 {
() => {
// Module: crate::appkit
// Provides: {"impl_149"}
// Dependencies: {}
impl NSSavePanel for id { unsafe fn setDirectoryURL (self , url : id) { msg_send ! [self , setDirectoryURL : url] } unsafe fn setCanCreateDirectories (self , canCreateDirectories : BOOL) { msg_send ! [self , setCanCreateDirectories : canCreateDirectories] } unsafe fn URL (self) -> id { msg_send ! [self , URL] } unsafe fn runModal (self) -> NSModalResponse { msg_send ! [self , runModal] } }
};
}
