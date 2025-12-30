// Generated macro for impl_14 (impl)
macro_rules! Depcrate_parseimpl_14 {
() => {
// Module: crate::parse
// Provides: {"impl_14"}
// Dependencies: {}
impl Properties { pub const fn unicode_version (& self) -> (u8 , u8 , u8) { self . unicode_version } pub fn is_xid_start (& self , ch : char) -> bool { self . xid_start . contains (& (ch as u32)) } pub fn is_xid_continue (& self , ch : char) -> bool { self . xid_continue . contains (& (ch as u32)) } }
};
}
