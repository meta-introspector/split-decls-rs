// Generated macro for impl_64 (impl)
macro_rules! Depcrate_bstrimpl_64 {
() => {
// Module: crate::bstr
// Provides: {"impl_64"}
// Dependencies: {}
impl Deref for BasicString { type Target = [u16] ; fn deref (& self) -> & [u16] { let len = if self . 0 . is_null () { 0 } else { unsafe { SysStringLen (self . 0) as usize } } ; if len > 0 { unsafe { core :: slice :: from_raw_parts (self . 0 , len) } } else { const EMPTY : [u16 ; 1] = [0] ; & EMPTY [.. 0] } } }
};
}
