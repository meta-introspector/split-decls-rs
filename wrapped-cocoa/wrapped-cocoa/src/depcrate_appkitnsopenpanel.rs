// Generated macro for NSOpenPanel (trait)
macro_rules! Depcrate_appkitNSOpenPanel {
() => {
// Module: crate::appkit
// Provides: {"NSOpenPanel"}
// Dependencies: {}
pub trait NSOpenPanel : NSSavePanel { unsafe fn openPanel (_ : Self) -> id { msg_send ! [class ! (NSOpenPanel) , openPanel] } unsafe fn setCanChooseFiles_ (self , canChooseFiles : BOOL) ; unsafe fn setCanChooseDirectories_ (self , canChooseDirectories : BOOL) ; unsafe fn setResolvesAliases_ (self , resolvesAliases : BOOL) ; unsafe fn setAllowsMultipleSelection_ (self , allowsMultipleSelection : BOOL) ; unsafe fn URLs (self) -> id ; }
};
}
