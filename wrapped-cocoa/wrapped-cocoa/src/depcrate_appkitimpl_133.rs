// Generated macro for impl_133 (impl)
macro_rules! Depcrate_appkitimpl_133 {
() => {
// Module: crate::appkit
// Provides: {"impl_133"}
// Dependencies: {}
impl NSPasteboardReading for id { unsafe fn initWithPasteboardPropertyList_ofType (self , propertyList : id , _type : id) -> id { msg_send ! [self , initWithPasteboardPropertyList : propertyList ofType : _type] } unsafe fn readableTypesForPasteboard (self , pasteboard : id) -> id { let class : id = msg_send ! [self , class] ; msg_send ! [class , readableTypesForPasteboard : pasteboard] } unsafe fn readingOptionsForType_pasteboard (self , _type : id , pasteboard : id ,) -> NSPasteboardReadingOptions { let class : id = msg_send ! [self , class] ; msg_send ! [class , readingOptionsForType : _type pasteboard : pasteboard] } }
};
}
