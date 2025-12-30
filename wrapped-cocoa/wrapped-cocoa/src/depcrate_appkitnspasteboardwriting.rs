// Generated macro for NSPasteboardWriting (trait)
macro_rules! Depcrate_appkitNSPasteboardWriting {
() => {
// Module: crate::appkit
// Provides: {"NSPasteboardWriting"}
// Dependencies: {}
pub trait NSPasteboardWriting : Sized { unsafe fn writableTypesForPasteboard (self , pasteboard : id) -> id ; unsafe fn writingOptionsForType_pasteboard (self , _type : id , pasteboard : id ,) -> NSPasteboardWritingOptions ; unsafe fn pasteboardPropertyListForType (self , _type : id) -> id ; }
};
}
