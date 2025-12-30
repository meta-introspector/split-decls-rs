// Generated macro for impl_801 (impl)
macro_rules! Depcrate_mkeymapimpl_801 {
() => {
// Module: crate::mkeymap
// Provides: {"impl_801"}
// Dependencies: {}
impl Index < & '_ KeyType > for MKeyMap { type Output = Arg ; fn index (& self , key : & KeyType) -> & Self :: Output { self . get (key) . expect (INTERNAL_ERROR_MSG) } }
};
}
