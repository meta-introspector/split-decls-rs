// Generated macro for impl_151 (impl)
macro_rules! Depcrate_appkitimpl_151 {
() => {
// Module: crate::appkit
// Provides: {"impl_151"}
// Dependencies: {}
impl NSOpenPanel for id { unsafe fn setCanChooseFiles_ (self , canChooseFiles : BOOL) { msg_send ! [self , setCanChooseFiles : canChooseFiles] } unsafe fn setCanChooseDirectories_ (self , canChooseDirectories : BOOL) { msg_send ! [self , setCanChooseDirectories : canChooseDirectories] } unsafe fn setResolvesAliases_ (self , resolvesAliases : BOOL) { msg_send ! [self , setResolvesAliases : resolvesAliases] } unsafe fn setAllowsMultipleSelection_ (self , allowsMultipleSelection : BOOL) { msg_send ! [self , setAllowsMultipleSelection : allowsMultipleSelection] } unsafe fn URLs (self) -> id { msg_send ! [self , URLs] } }
};
}
