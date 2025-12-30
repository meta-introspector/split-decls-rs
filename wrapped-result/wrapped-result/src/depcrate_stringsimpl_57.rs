// Generated macro for impl_57 (impl)
macro_rules! Depcrate_stringsimpl_57 {
() => {
// Module: crate::strings
// Provides: {"impl_57"}
// Dependencies: {}
impl Drop for HeapString { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { HeapFree (GetProcessHeap () , 0 , self . 0 as _) ; } } } }
};
}
