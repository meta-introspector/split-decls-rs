// Generated macro for impl_145 (impl)
macro_rules! Depcrate_comimpl_145 {
() => {
// Module: crate::com
// Provides: {"impl_145"}
// Dependencies: {}
impl BStr { pub unsafe fn from_raw (s : BSTR) -> BStr { BStr (s) } pub fn to_osstring (& self) -> OsString { let len = unsafe { SysStringLen (self . 0) } ; let slice = unsafe { from_raw_parts (self . 0 , len as usize) } ; OsStringExt :: from_wide (slice) } }
};
}
