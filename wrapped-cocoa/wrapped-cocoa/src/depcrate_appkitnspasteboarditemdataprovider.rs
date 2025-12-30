// Generated macro for NSPasteboardItemDataProvider (trait)
macro_rules! Depcrate_appkitNSPasteboardItemDataProvider {
() => {
// Module: crate::appkit
// Provides: {"NSPasteboardItemDataProvider"}
// Dependencies: {}
pub trait NSPasteboardItemDataProvider : Sized { unsafe fn pasteboard_item_provideDataForType (self , pasteboard : id , item : id , _type : id) ; unsafe fn pasteboardFinishedWithDataProvider (self , pasteboard : id) ; }
};
}
