// Generated macro for NSPasteboardReading (trait)
macro_rules! Depcrate_appkitNSPasteboardReading {
() => {
// Module: crate::appkit
// Provides: {"NSPasteboardReading"}
// Dependencies: {}
pub trait NSPasteboardReading : Sized { unsafe fn initWithPasteboardPropertyList_ofType (self , propertyList : id , _type : id) -> id ; unsafe fn readableTypesForPasteboard (self , pasteboard : id) -> id ; unsafe fn readingOptionsForType_pasteboard (self , _type : id , pasteboard : id ,) -> NSPasteboardReadingOptions ; }
};
}
