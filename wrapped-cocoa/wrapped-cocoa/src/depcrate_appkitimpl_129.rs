// Generated macro for impl_129 (impl)
macro_rules! Depcrate_appkitimpl_129 {
() => {
// Module: crate::appkit
// Provides: {"impl_129"}
// Dependencies: {}
impl NSPasteboardItemDataProvider for id { unsafe fn pasteboard_item_provideDataForType (self , pasteboard : id , item : id , _type : id) { msg_send ! [self , pasteboard : pasteboard item : item provideDataForType : _type] } unsafe fn pasteboardFinishedWithDataProvider (self , pasteboard : id) { msg_send ! [self , pasteboardFinishedWithDataProvider : pasteboard] } }
};
}
