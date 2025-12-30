// Generated macro for NSPasteboardItem (trait)
macro_rules! Depcrate_appkitNSPasteboardItem {
() => {
// Module: crate::appkit
// Provides: {"NSPasteboardItem"}
// Dependencies: {}
pub trait NSPasteboardItem : Sized { unsafe fn types (self) -> id ; unsafe fn setDataProvider_forTypes (self , dataProvider : id , types : id) -> BOOL ; unsafe fn setData_forType (self , data : id , _type : id) -> BOOL ; unsafe fn setString_forType (self , string : id , _type : id) -> BOOL ; unsafe fn setPropertyList_forType (self , propertyList : id , _type : id) -> BOOL ; unsafe fn dataForType (self , _type : id) -> id ; unsafe fn stringForType (self , _type : id) -> id ; unsafe fn propertyListForType (self , _type : id) -> id ; }
};
}
