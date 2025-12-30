// Generated macro for impl_131 (impl)
macro_rules! Depcrate_appkitimpl_131 {
() => {
// Module: crate::appkit
// Provides: {"impl_131"}
// Dependencies: {}
impl NSPasteboardWriting for id { unsafe fn writableTypesForPasteboard (self , pasteboard : id) -> id { msg_send ! [self , writableTypesForPasteboard : pasteboard] } unsafe fn writingOptionsForType_pasteboard (self , _type : id , pasteboard : id ,) -> NSPasteboardWritingOptions { msg_send ! [self , writingOptionsForType : _type pasteboard : pasteboard] } unsafe fn pasteboardPropertyListForType (self , _type : id) -> id { msg_send ! [self , pasteboardPropertyListForType : _type] } }
};
}
