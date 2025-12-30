// Generated macro for impl_127 (impl)
macro_rules! Depcrate_appkitimpl_127 {
() => {
// Module: crate::appkit
// Provides: {"impl_127"}
// Dependencies: {}
impl NSPasteboardItem for id { unsafe fn types (self) -> id { msg_send ! [self , types] } unsafe fn setDataProvider_forTypes (self , dataProvider : id , types : id) -> BOOL { msg_send ! [self , setDataProvider : dataProvider forTypes : types] } unsafe fn setData_forType (self , data : id , _type : id) -> BOOL { msg_send ! [self , setData : data forType : _type] } unsafe fn setString_forType (self , string : id , _type : id) -> BOOL { msg_send ! [self , setString : string forType : _type] } unsafe fn setPropertyList_forType (self , propertyList : id , _type : id) -> BOOL { msg_send ! [self , setPropertyList : propertyList forType : _type] } unsafe fn dataForType (self , _type : id) -> id { msg_send ! [self , dataForType : _type] } unsafe fn stringForType (self , _type : id) -> id { msg_send ! [self , stringForType : _type] } unsafe fn propertyListForType (self , _type : id) -> id { msg_send ! [self , propertyListForType : _type] } }
};
}
