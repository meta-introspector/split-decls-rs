// Generated macro for impl_70 (impl)
macro_rules! Depcrate_foundationimpl_70 {
() => {
// Module: crate::foundation
// Provides: {"impl_70"}
// Dependencies: {}
impl NSUserDefaults for id { unsafe fn standardUserDefaults () -> id { msg_send ! [class ! (NSUserDefaults) , standardUserDefaults] } unsafe fn setBool_forKey_ (self , value : BOOL , key : id) { msg_send ! [self , setBool : value forKey : key] } unsafe fn bool_forKey_ (self , key : id) -> BOOL { msg_send ! [self , boolForKey : key] } unsafe fn removeObject_forKey_ (self , key : id) { msg_send ! [self , removeObjectForKey : key] } }
};
}
