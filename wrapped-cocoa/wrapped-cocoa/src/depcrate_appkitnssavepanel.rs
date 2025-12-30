// Generated macro for NSSavePanel (trait)
macro_rules! Depcrate_appkitNSSavePanel {
() => {
// Module: crate::appkit
// Provides: {"NSSavePanel"}
// Dependencies: {}
pub trait NSSavePanel : Sized { unsafe fn savePanel (_ : Self) -> id { msg_send ! [class ! (NSSavePanel) , savePanel] } unsafe fn setDirectoryURL (self , url : id) ; unsafe fn setCanCreateDirectories (self , canCreateDirectories : BOOL) ; unsafe fn URL (self) -> id ; unsafe fn runModal (self) -> NSModalResponse ; }
};
}
